//! Per-actor GC-aware heap with free-list allocator.
//!
//! Each Mesh actor gets its own heap for memory allocation. This eliminates
//! global arena contention and enables per-actor memory reclamation via
//! mark-sweep garbage collection.
//!
//! Every allocation prepends a 16-byte `GcHeader` before the user data.
//! All live objects are linked via an intrusive all-objects list for sweep
//! traversal. Freed blocks are placed on a free list for reuse before
//! bump-allocating new pages.

use std::ptr;

/// Default page size for actor heaps: 64 KiB.
const ACTOR_PAGE_SIZE: usize = 64 * 1024;

// ---------------------------------------------------------------------------
// GcHeader
// ---------------------------------------------------------------------------

/// Size of the GcHeader in bytes.
pub const GC_HEADER_SIZE: usize = 16;

/// Mark bit in GcHeader flags: object is reachable (set during mark phase).
pub const MARK_BIT: u8 = 0x01;

/// Free bit in GcHeader flags: object is on the free list.
pub const FREE_BIT: u8 = 0x02;

/// Object header prepended to every GC-managed allocation.
///
/// The user-visible pointer starts immediately after this header.
/// The `next` pointer serves dual purpose: when the object is live, it links
/// into the all-objects list; when freed, it links into the free list.
#[repr(C)]
pub struct GcHeader {
    /// Size of the user data in bytes (not including the header).
    pub size: u32,
    /// Flags: bit 0 = marked, bit 1 = free.
    pub flags: u8,
    /// Reserved padding for 8-byte alignment of the `next` pointer.
    pub _pad: [u8; 3],
    /// Next pointer: links into the all-objects list or free list.
    pub next: *mut GcHeader,
}

// GcHeader contains a raw pointer but is only used within a single actor's
// heap (never shared across threads). Mark as Send so ActorHeap can be Send.
unsafe impl Send for GcHeader {}

impl GcHeader {
    /// Returns true if the mark bit is set.
    #[inline]
    pub fn is_marked(&self) -> bool {
        self.flags & MARK_BIT != 0
    }

    /// Set the mark bit.
    #[inline]
    pub fn set_marked(&mut self) {
        self.flags |= MARK_BIT;
    }

    /// Clear the mark bit.
    #[inline]
    pub fn clear_marked(&mut self) {
        self.flags &= !MARK_BIT;
    }

    /// Returns true if the free bit is set.
    #[inline]
    pub fn is_free(&self) -> bool {
        self.flags & FREE_BIT != 0
    }

    /// Set the free bit.
    #[inline]
    pub fn set_free(&mut self) {
        self.flags |= FREE_BIT;
    }

    /// Clear the free bit.
    #[inline]
    pub fn clear_free(&mut self) {
        self.flags &= !FREE_BIT;
    }

    /// Returns a pointer to the user data (past the header).
    #[inline]
    pub fn data_ptr(&mut self) -> *mut u8 {
        unsafe { (self as *mut GcHeader as *mut u8).add(GC_HEADER_SIZE) }
    }

    /// Recover the GcHeader pointer from a user data pointer.
    ///
    /// # Safety
    ///
    /// `data` must point to user data that was allocated via `ActorHeap::alloc`,
    /// i.e., it must have a valid GcHeader immediately preceding it.
    #[inline]
    pub unsafe fn from_data_ptr(data: *mut u8) -> *mut GcHeader {
        data.sub(GC_HEADER_SIZE) as *mut GcHeader
    }
}

// ---------------------------------------------------------------------------
// ActorHeap
// ---------------------------------------------------------------------------

/// Default GC pressure threshold: 256 KiB.
const DEFAULT_GC_THRESHOLD: usize = 256 * 1024;

/// Per-actor heap with GcHeader-prepended free-list allocator.
///
/// Owns a list of pages and bump-allocates within the current page.
/// Every allocation prepends a 16-byte `GcHeader` and links the object
/// into the `all_objects` intrusive list. Freed blocks are placed on the
/// `free_list` for reuse before bump-allocating new pages.
pub struct ActorHeap {
    /// Owned page list. Each page is a heap-allocated byte buffer.
    pages: Vec<Vec<u8>>,
    /// Bump offset into the current (last) page.
    offset: usize,
    /// Total bytes allocated (including headers) for GC trigger heuristics.
    total_allocated: usize,

    /// Head of the intrusive all-objects linked list (for sweep traversal).
    all_objects: *mut GcHeader,
    /// Head of the free list (freed blocks available for reuse).
    free_list: *mut GcHeader,

    /// Heap pressure threshold in bytes. When `total_allocated >= gc_threshold`,
    /// the GC should be triggered.
    gc_threshold: usize,
    /// Re-entrancy guard: prevents GC from triggering during GC.
    gc_in_progress: bool,
}

// Raw pointers in ActorHeap are only accessed from the owning actor's thread.
unsafe impl Send for ActorHeap {}

impl ActorHeap {
    /// Create a new per-actor heap with an initial 64 KiB page.
    pub fn new() -> Self {
        let mut heap = ActorHeap {
            pages: Vec::new(),
            offset: 0,
            total_allocated: 0,
            all_objects: ptr::null_mut(),
            free_list: ptr::null_mut(),
            gc_threshold: DEFAULT_GC_THRESHOLD,
            gc_in_progress: false,
        };
        heap.pages.push(vec![0u8; ACTOR_PAGE_SIZE]);
        heap
    }

    /// Allocate `size` bytes with the given `align`ment.
    ///
    /// Returns a pointer to zeroed memory within this actor's heap.
    /// The pointer is past the GcHeader -- callers see only user data.
    /// The pointer is valid until the object is collected or `reset()` is called.
    pub fn alloc(&mut self, size: usize, align: usize) -> *mut u8 {
        let align = if align == 0 { 1 } else { align };

        // 1. Try the free list first: find a block with sufficient size.
        if let Some(data_ptr) = self.alloc_from_free_list(size) {
            return data_ptr;
        }

        // 2. Bump-allocate: GcHeader + user data from pages.
        self.bump_alloc_with_header(size, align)
    }

    /// Try to allocate from the free list (first-fit).
    ///
    /// Walks the free list looking for a block where `header.size >= size`.
    /// If found, unlinks from free list, clears FREE_BIT, links into
    /// all_objects, and returns the user data pointer.
    fn alloc_from_free_list(&mut self, size: usize) -> Option<*mut u8> {
        let mut current = self.free_list;
        let mut prev: *mut GcHeader = ptr::null_mut();

        while !current.is_null() {
            let header = unsafe { &mut *current };
            if header.size as usize >= size {
                // Found a suitable block. Unlink from free list.
                let next = header.next;
                if !prev.is_null() {
                    unsafe {
                        (*prev).next = next;
                    }
                } else {
                    self.free_list = next;
                }

                // Clear the free bit, zero flags, link into all_objects list.
                header.flags = 0;
                header.next = self.all_objects;
                self.all_objects = current;

                // Zero the user data region for safety.
                let data = header.data_ptr();
                unsafe {
                    ptr::write_bytes(data, 0, header.size as usize);
                }

                return Some(data);
            }
            prev = current;
            current = header.next;
        }

        None
    }

    /// Bump-allocate `GC_HEADER_SIZE + size` bytes from pages and initialize
    /// the GcHeader.
    fn bump_alloc_with_header(&mut self, size: usize, align: usize) -> *mut u8 {
        let total = GC_HEADER_SIZE + size;

        if self.pages.is_empty() {
            self.pages.push(vec![0u8; ACTOR_PAGE_SIZE]);
            self.offset = 0;
        }

        // We need the USER DATA pointer (header + GC_HEADER_SIZE) to satisfy
        // the requested alignment. Compute where the data would land and work
        // backwards to find the header offset.

        // Try to fit in the current page.
        let page_base = self.pages.last().unwrap().as_ptr() as usize;
        let current_page_len = self.pages.last().map_or(0, |p| p.len());

        // The earliest the data could start is at offset + GC_HEADER_SIZE.
        let data_addr = page_base + self.offset + GC_HEADER_SIZE;
        let aligned_data_addr = (data_addr + align - 1) & !(align - 1);
        let header_offset = aligned_data_addr - GC_HEADER_SIZE - page_base;

        if header_offset + total <= current_page_len {
            // Fits in the current page.
            let page = self.pages.last_mut().unwrap();
            let header_ptr = page[header_offset..].as_mut_ptr() as *mut GcHeader;

            // Initialize the GcHeader.
            unsafe {
                (*header_ptr).size = size as u32;
                (*header_ptr).flags = 0;
                (*header_ptr)._pad = [0; 3];
                (*header_ptr).next = self.all_objects;
            }
            self.all_objects = header_ptr;

            self.offset = header_offset + total;
            self.total_allocated += total;

            unsafe { (*header_ptr).data_ptr() }
        } else {
            // Allocate a new page. If the total exceeds the default page size,
            // allocate a page large enough (with room for alignment padding).
            let max_padding = if align > GC_HEADER_SIZE { align } else { 0 };
            let new_page_size = if total + max_padding > ACTOR_PAGE_SIZE {
                total + max_padding
            } else {
                ACTOR_PAGE_SIZE
            };
            let mut page = vec![0u8; new_page_size];

            // Align the user data pointer, then back up for the header.
            let new_base = page.as_ptr() as usize;
            let new_data_addr = new_base + GC_HEADER_SIZE;
            let aligned_new_data = (new_data_addr + align - 1) & !(align - 1);
            let new_header_offset = aligned_new_data - GC_HEADER_SIZE - new_base;

            let header_ptr = page[new_header_offset..].as_mut_ptr() as *mut GcHeader;

            // Initialize the GcHeader.
            unsafe {
                (*header_ptr).size = size as u32;
                (*header_ptr).flags = 0;
                (*header_ptr)._pad = [0; 3];
                (*header_ptr).next = self.all_objects;
            }
            self.all_objects = header_ptr;

            self.offset = new_header_offset + total;
            self.total_allocated += total;
            self.pages.push(page);

            unsafe { (*header_ptr).data_ptr() }
        }
    }

    /// Drop all pages and start fresh.
    ///
    /// Used for actor termination cleanup or after full GC sweep.
    pub fn reset(&mut self) {
        self.pages.clear();
        self.offset = 0;
        self.total_allocated = 0;
        self.all_objects = ptr::null_mut();
        self.free_list = ptr::null_mut();
    }

    /// Returns the total number of bytes allocated from this heap
    /// (including GcHeader overhead).
    pub fn total_bytes(&self) -> usize {
        self.total_allocated
    }

    /// Returns true if the heap has exceeded its GC pressure threshold.
    pub fn should_collect(&self) -> bool {
        !self.gc_in_progress && self.total_allocated >= self.gc_threshold
    }

    /// Returns a pointer to the head of the all-objects list.
    pub fn all_objects_head(&self) -> *mut GcHeader {
        self.all_objects
    }

    /// Returns a pointer to the head of the free list.
    pub fn free_list_head(&self) -> *mut GcHeader {
        self.free_list
    }

    /// Returns whether GC is currently in progress.
    pub fn gc_in_progress(&self) -> bool {
        self.gc_in_progress
    }

    /// Set the GC-in-progress flag.
    pub fn set_gc_in_progress(&mut self, value: bool) {
        self.gc_in_progress = value;
    }

    /// Set the all-objects head pointer (used by sweep phase).
    pub fn set_all_objects_head(&mut self, head: *mut GcHeader) {
        self.all_objects = head;
    }

    /// Add a header to the free list (used by sweep phase).
    pub fn add_to_free_list(&mut self, header: *mut GcHeader) {
        unsafe {
            (*header).next = self.free_list;
        }
        self.free_list = header;
    }

    /// Returns the GC threshold in bytes.
    pub fn gc_threshold(&self) -> usize {
        self.gc_threshold
    }

    /// Set the GC threshold in bytes.
    pub fn set_gc_threshold(&mut self, threshold: usize) {
        self.gc_threshold = threshold;
    }

    /// Subtract from total_allocated (used after sweep frees objects).
    pub fn subtract_allocated(&mut self, bytes: usize) {
        self.total_allocated = self.total_allocated.saturating_sub(bytes);
    }

    // -----------------------------------------------------------------------
    // Mark-Sweep Garbage Collection
    // -----------------------------------------------------------------------

    /// Run a full mark-sweep garbage collection cycle.
    ///
    /// Conservatively scans the coroutine stack between `stack_bottom` and
    /// `stack_top` for roots, marks all transitively reachable objects, then
    /// sweeps unreachable objects onto the free list.
    ///
    /// `stack_top` has the lower address (stack grows downward on x86-64/ARM64).
    /// `stack_bottom` has the higher address (the base of the coroutine stack).
    ///
    /// This method is guarded against re-entrancy: if `gc_in_progress` is
    /// already set, the call is a no-op.
    pub fn collect(&mut self, stack_bottom: *const u8, stack_top: *const u8) {
        if self.gc_in_progress {
            return;
        }
        self.gc_in_progress = true;

        self.mark_from_roots(stack_bottom, stack_top);
        self.sweep();

        self.gc_in_progress = false;
    }

    /// Mark phase: conservatively scan the stack and trace all reachable objects.
    ///
    /// 1. Walk the stack from `stack_top` (low address) to `stack_bottom`
    ///    (high address), treating every 8-byte-aligned word as a potential
    ///    pointer. If it points into a live object in this heap, mark it as
    ///    a root.
    ///
    /// 2. Process a worklist (tricolor marking): for each marked object, scan
    ///    its body for further heap pointers and mark those transitively.
    ///
    /// The worklist is a `Vec` allocated on the system heap (via Rust's
    /// allocator), NOT on the GC heap, to avoid re-entrancy issues.
    fn mark_from_roots(&mut self, stack_bottom: *const u8, stack_top: *const u8) {
        // Worklist lives on the system heap (Rust Vec -> malloc).
        let mut worklist: Vec<*mut GcHeader> = Vec::new();

        // Ensure stack_top <= stack_bottom (stack_top is lower address).
        let (lo, hi) = if (stack_top as usize) <= (stack_bottom as usize) {
            (stack_top as usize, stack_bottom as usize)
        } else {
            (stack_bottom as usize, stack_top as usize)
        };

        // Phase 1: Conservative stack scanning.
        // Walk every 8-byte-aligned word in the stack range.
        let aligned_lo = (lo + 7) & !7; // round up to 8-byte alignment
        let mut addr = aligned_lo;
        while addr + 8 <= hi {
            let word = unsafe { *(addr as *const usize) };
            if let Some(header) = self.find_object_containing(word as *const u8) {
                let hdr = unsafe { &mut *header };
                if !hdr.is_marked() {
                    hdr.set_marked();
                    worklist.push(header);
                }
            }
            addr += 8;
        }

        // Phase 2: Worklist-based transitive marking (tricolor).
        while let Some(header) = worklist.pop() {
            let hdr = unsafe { &*header };
            let data_start = unsafe { (header as *mut u8).add(GC_HEADER_SIZE) };
            let body_size = hdr.size as usize;

            // Scan every 8-byte word in the object body.
            let mut offset = 0;
            while offset + 8 <= body_size {
                let word = unsafe { *(data_start.add(offset) as *const usize) };
                if let Some(target_header) = self.find_object_containing(word as *const u8) {
                    let target = unsafe { &mut *target_header };
                    if !target.is_marked() {
                        target.set_marked();
                        worklist.push(target_header);
                    }
                }
                offset += 8;
            }
        }
    }

    /// Check if `ptr` points into a live (non-free) object in this heap.
    ///
    /// First does a quick page-range check (is `ptr` within any page's
    /// address range?). If yes, walks the all-objects list to find an object
    /// whose data range `[data_ptr, data_ptr + size)` contains `ptr`.
    ///
    /// This handles interior pointers: a pointer anywhere within an object's
    /// body identifies that object as reachable.
    ///
    /// Returns `Some(header_ptr)` if found, `None` otherwise.
    fn find_object_containing(&self, ptr: *const u8) -> Option<*mut GcHeader> {
        let ptr_addr = ptr as usize;

        // Quick check: is ptr within any page's address range?
        let in_any_page = self.pages.iter().any(|page| {
            let page_start = page.as_ptr() as usize;
            let page_end = page_start + page.len();
            ptr_addr >= page_start && ptr_addr < page_end
        });

        if !in_any_page {
            return None;
        }

        // Walk the all-objects list to find a live object containing ptr.
        let mut current = self.all_objects;
        while !current.is_null() {
            let header = unsafe { &*current };
            // Skip free objects -- a pointer to freed memory is not a root.
            if !header.is_free() {
                let data_start = unsafe { (current as *const u8).add(GC_HEADER_SIZE) } as usize;
                let data_end = data_start + header.size as usize;
                if ptr_addr >= data_start && ptr_addr < data_end {
                    return Some(current);
                }
            }
            current = header.next;
        }

        None
    }

    /// Sweep phase: walk the all-objects list and free unmarked objects.
    ///
    /// For each object in the all-objects list:
    /// - If marked: clear the mark bit, keep in the list.
    /// - If NOT marked: unlink from the list, set FREE_BIT, add to free_list,
    ///   and subtract its size from `total_allocated`.
    ///
    /// Rebuilds the all-objects list in-place using a prev-pointer technique.
    fn sweep(&mut self) {
        let mut current = self.all_objects;
        let mut prev: *mut GcHeader = ptr::null_mut();
        let mut new_head = self.all_objects;
        let mut first = true;

        while !current.is_null() {
            let header = unsafe { &mut *current };
            let next = header.next;

            if header.is_marked() {
                // Reachable: clear mark bit and keep in list.
                header.clear_marked();
                if first {
                    new_head = current;
                    first = false;
                }
                prev = current;
                current = next;
            } else {
                // Unreachable: unlink from all_objects and add to free list.
                let freed_bytes = GC_HEADER_SIZE + header.size as usize;
                self.total_allocated = self.total_allocated.saturating_sub(freed_bytes);

                // Unlink from all_objects list.
                if !prev.is_null() {
                    unsafe {
                        (*prev).next = next;
                    }
                } else {
                    // We're removing the head.
                    new_head = next;
                }

                // Add to free list.
                header.set_free();
                header.next = self.free_list;
                self.free_list = current;

                current = next;
                // prev stays the same -- we removed current.
            }
        }

        self.all_objects = if first { ptr::null_mut() } else { new_head };
    }
}

impl Default for ActorHeap {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for ActorHeap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActorHeap")
            .field("pages", &self.pages.len())
            .field("offset", &self.offset)
            .field("total_allocated", &self.total_allocated)
            .field("all_objects", &(!self.all_objects.is_null()))
            .field("free_list", &(!self.free_list.is_null()))
            .field("gc_threshold", &self.gc_threshold)
            .field("gc_in_progress", &self.gc_in_progress)
            .finish()
    }
}

// ---------------------------------------------------------------------------
// MessageBuffer
// ---------------------------------------------------------------------------

/// A serialized message representation for cross-heap copying.
///
/// When an actor sends a message to another actor, the data is serialized
/// into a `MessageBuffer` and then deep-copied into the target actor's heap.
/// This ensures complete isolation between actor heaps.
#[derive(Debug, Clone)]
pub struct MessageBuffer {
    /// Raw serialized message bytes.
    pub data: Vec<u8>,
    /// Type tag for pattern matching dispatch.
    ///
    /// In Phase 6, this is a simple hash of the type name. Future phases
    /// may use a more sophisticated type identification scheme.
    pub type_tag: u64,
}

impl MessageBuffer {
    /// Create a new message buffer from raw bytes and a type tag.
    pub fn new(data: Vec<u8>, type_tag: u64) -> Self {
        MessageBuffer { data, type_tag }
    }

    /// Deep-copy this message's data into the target actor's heap.
    ///
    /// Allocates space in the target heap (with GcHeader prepended
    /// automatically), copies the data bytes, and returns a pointer
    /// to the copy within the target heap.
    pub fn deep_copy_to_heap(&self, heap: &mut ActorHeap) -> *mut u8 {
        if self.data.is_empty() {
            return std::ptr::null_mut();
        }
        let ptr = heap.alloc(self.data.len(), 8);
        // Safety: ptr points to a valid allocation of at least self.data.len() bytes.
        unsafe {
            std::ptr::copy_nonoverlapping(self.data.as_ptr(), ptr, self.data.len());
        }
        ptr
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gc_header_layout() {
        // GcHeader must be exactly 16 bytes.
        assert_eq!(
            std::mem::size_of::<GcHeader>(),
            GC_HEADER_SIZE,
            "GcHeader must be exactly 16 bytes"
        );

        // Verify data_ptr / from_data_ptr round-trip.
        let mut heap = ActorHeap::new();
        let data_ptr = heap.alloc(64, 8);
        assert!(!data_ptr.is_null());

        let header_ptr = unsafe { GcHeader::from_data_ptr(data_ptr) };
        assert!(!header_ptr.is_null());

        let recovered_data = unsafe { (*header_ptr).data_ptr() };
        assert_eq!(
            data_ptr, recovered_data,
            "data_ptr/from_data_ptr round-trip"
        );

        // Verify header fields.
        let header = unsafe { &*header_ptr };
        assert_eq!(header.size, 64);
        assert_eq!(header.flags, 0);
        assert!(!header.is_marked());
        assert!(!header.is_free());
    }

    #[test]
    fn test_gc_header_flags() {
        let mut header = GcHeader {
            size: 100,
            flags: 0,
            _pad: [0; 3],
            next: ptr::null_mut(),
        };

        // Mark bit.
        assert!(!header.is_marked());
        header.set_marked();
        assert!(header.is_marked());
        assert!(!header.is_free());
        header.clear_marked();
        assert!(!header.is_marked());

        // Free bit.
        assert!(!header.is_free());
        header.set_free();
        assert!(header.is_free());
        assert!(!header.is_marked());
        header.clear_free();
        assert!(!header.is_free());

        // Both bits.
        header.set_marked();
        header.set_free();
        assert!(header.is_marked());
        assert!(header.is_free());
        assert_eq!(header.flags, MARK_BIT | FREE_BIT);
    }

    #[test]
    fn test_actor_heap_basic_alloc() {
        let mut heap = ActorHeap::new();
        let ptr1 = heap.alloc(16, 8);
        assert!(!ptr1.is_null());

        let ptr2 = heap.alloc(32, 8);
        assert!(!ptr2.is_null());

        // Pointers should be different.
        assert_ne!(ptr1, ptr2);
    }

    #[test]
    fn test_actor_heap_alignment() {
        let mut heap = ActorHeap::new();

        // Test various alignments.
        for &align in &[1, 2, 4, 8, 16, 32, 64] {
            let ptr = heap.alloc(8, align);
            assert!(!ptr.is_null());
            assert_eq!(
                ptr as usize % align,
                0,
                "pointer should be {}-byte aligned, got {:p}",
                align,
                ptr
            );
        }
    }

    #[test]
    fn test_actor_heap_large_alloc() {
        let mut heap = ActorHeap::new();
        // Allocate more than a page.
        let ptr = heap.alloc(128 * 1024, 8);
        assert!(!ptr.is_null());
        assert!(heap.pages.len() >= 2, "should have allocated a new page");
    }

    #[test]
    fn test_actor_heap_reset() {
        let mut heap = ActorHeap::new();

        // Allocate some memory.
        heap.alloc(1024, 8);
        heap.alloc(2048, 8);
        assert!(heap.total_bytes() > 0);
        assert!(!heap.pages.is_empty());

        // Reset should clear everything including GC lists.
        heap.reset();
        assert_eq!(heap.total_bytes(), 0);
        assert!(heap.pages.is_empty());
        assert_eq!(heap.offset, 0);
        assert!(heap.all_objects.is_null());
        assert!(heap.free_list.is_null());
    }

    #[test]
    fn test_actor_heap_total_bytes() {
        let mut heap = ActorHeap::new();
        assert_eq!(heap.total_bytes(), 0);

        // Each alloc adds GC_HEADER_SIZE + requested size.
        heap.alloc(100, 8);
        assert_eq!(heap.total_bytes(), GC_HEADER_SIZE + 100);

        heap.alloc(200, 8);
        assert_eq!(heap.total_bytes(), 2 * GC_HEADER_SIZE + 300);
    }

    #[test]
    fn test_all_objects_list() {
        let mut heap = ActorHeap::new();

        // Allocate 3 objects.
        let _p1 = heap.alloc(32, 8);
        let _p2 = heap.alloc(64, 8);
        let _p3 = heap.alloc(16, 8);

        // Walk the all_objects list and count entries.
        let mut count = 0;
        let mut current = heap.all_objects_head();
        while !current.is_null() {
            count += 1;
            let header = unsafe { &*current };
            assert!(!header.is_free());
            assert!(!header.is_marked());
            current = header.next;
        }
        assert_eq!(count, 3, "all_objects list should contain 3 objects");
    }

    #[test]
    fn test_free_list_reuse() {
        let mut heap = ActorHeap::new();

        // Allocate an object.
        let ptr1 = heap.alloc(64, 8);
        assert!(!ptr1.is_null());
        let header1 = unsafe { GcHeader::from_data_ptr(ptr1) };

        // Record total_allocated after first alloc.
        let allocated_after_first = heap.total_bytes();

        // Manually free it: unlink from all_objects, set FREE, add to free list.
        // (In normal GC, sweep does this; here we simulate.)
        let next_in_all = unsafe { (*header1).next };
        heap.set_all_objects_head(next_in_all);
        unsafe {
            (*header1).set_free();
        }
        heap.add_to_free_list(header1);

        // Allocate the same size -- should reuse from free list.
        let ptr2 = heap.alloc(64, 8);
        assert!(!ptr2.is_null());

        // The reused block should be the same memory region.
        assert_eq!(ptr1, ptr2, "free-list reuse should return the same pointer");

        // total_allocated should not have grown (free list reuse doesn't add).
        assert_eq!(heap.total_bytes(), allocated_after_first);

        // The header should no longer be free.
        let header2 = unsafe { &*GcHeader::from_data_ptr(ptr2) };
        assert!(!header2.is_free());
        assert_eq!(header2.size, 64);
    }

    #[test]
    fn test_free_list_larger_block() {
        let mut heap = ActorHeap::new();

        // Allocate a large block and free it.
        let ptr_big = heap.alloc(256, 8);
        let header_big = unsafe { GcHeader::from_data_ptr(ptr_big) };

        // Unlink from all_objects, add to free list.
        let next = unsafe { (*header_big).next };
        heap.set_all_objects_head(next);
        unsafe {
            (*header_big).set_free();
        }
        heap.add_to_free_list(header_big);

        // Allocate a smaller block -- should reuse the larger freed block.
        let ptr_small = heap.alloc(64, 8);
        assert_eq!(
            ptr_big, ptr_small,
            "should reuse larger free block for smaller request"
        );

        let header = unsafe { &*GcHeader::from_data_ptr(ptr_small) };
        // Size in the header remains the original (256), not the requested (64).
        assert_eq!(header.size, 256);
    }

    #[test]
    fn test_should_collect() {
        let mut heap = ActorHeap::new();
        heap.set_gc_threshold(100);

        assert!(!heap.should_collect());

        // Allocate enough to exceed the threshold.
        // Each alloc adds GC_HEADER_SIZE + size.
        heap.alloc(50, 8); // 66 bytes
        assert!(!heap.should_collect());

        heap.alloc(50, 8); // 66 more = 132 total, exceeds 100
        assert!(heap.should_collect());

        // When GC is in progress, should_collect returns false.
        heap.set_gc_in_progress(true);
        assert!(!heap.should_collect());
    }

    #[test]
    fn test_message_buffer_deep_copy() {
        let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
        let msg = MessageBuffer::new(data.clone(), 42);

        let mut target_heap = ActorHeap::new();
        let ptr = msg.deep_copy_to_heap(&mut target_heap);

        assert!(!ptr.is_null());

        // Verify the copied data matches.
        let copied = unsafe { std::slice::from_raw_parts(ptr, data.len()) };
        assert_eq!(copied, &data[..]);

        // Verify the GcHeader is present.
        let header = unsafe { &*GcHeader::from_data_ptr(ptr) };
        assert_eq!(header.size as usize, data.len());
        assert!(!header.is_free());
    }

    #[test]
    fn test_message_buffer_empty_data() {
        let msg = MessageBuffer::new(Vec::new(), 0);
        let mut target_heap = ActorHeap::new();
        let ptr = msg.deep_copy_to_heap(&mut target_heap);
        assert!(ptr.is_null());
    }

    #[test]
    fn test_message_buffer_deep_copy_isolation() {
        // Verify that modifying the source buffer after copy does not affect
        // the data in the target heap.
        let mut data = vec![10u8, 20, 30, 40];
        let msg = MessageBuffer::new(data.clone(), 99);

        let mut target_heap = ActorHeap::new();
        let ptr = msg.deep_copy_to_heap(&mut target_heap);

        // Mutate the original data.
        data[0] = 255;

        // The copied data should be unchanged.
        let copied = unsafe { std::slice::from_raw_parts(ptr, 4) };
        assert_eq!(copied, &[10, 20, 30, 40]);
    }

    // -----------------------------------------------------------------------
    // Mark-Sweep GC Tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_collect_frees_unreachable() {
        // Allocate 5 objects, don't reference them from the stack.
        // Collect with an empty stack range -- all should be freed.
        let mut heap = ActorHeap::new();
        let _p1 = heap.alloc(32, 8);
        let _p2 = heap.alloc(64, 8);
        let _p3 = heap.alloc(16, 8);
        let _p4 = heap.alloc(48, 8);
        let _p5 = heap.alloc(24, 8);

        assert!(heap.total_bytes() > 0);

        // Use an empty stack range (both pointers equal) so no roots are found.
        let dummy: u64 = 0;
        let stack_ptr = &dummy as *const u64 as *const u8;
        heap.collect(stack_ptr, stack_ptr);

        // All objects should have been swept to the free list.
        assert!(
            heap.all_objects_head().is_null(),
            "all_objects should be empty after collecting unreachable objects"
        );
        assert!(
            !heap.free_list_head().is_null(),
            "free_list should be non-empty after sweep"
        );
        assert_eq!(
            heap.total_bytes(),
            0,
            "total_allocated should be 0 after collecting all unreachable objects"
        );
    }

    #[test]
    fn test_collect_retains_reachable() {
        // Allocate an object, create a fake stack frame containing its pointer,
        // then collect. The object should NOT be freed.
        let mut heap = ActorHeap::new();
        let ptr = heap.alloc(64, 8);
        let original_total = heap.total_bytes();

        // Create a fake stack frame: an array containing the pointer value.
        // The GC will scan this as the stack and find the pointer.
        let fake_stack: [usize; 4] = [0, ptr as usize, 0, 0];
        let stack_bottom = unsafe {
            (&fake_stack[0] as *const usize as *const u8).add(std::mem::size_of_val(&fake_stack))
        };
        let stack_top = &fake_stack[0] as *const usize as *const u8;

        heap.collect(stack_bottom, stack_top);

        // The object should be retained (reachable from the fake stack).
        assert!(
            !heap.all_objects_head().is_null(),
            "reachable object should survive GC"
        );
        assert_eq!(
            heap.total_bytes(),
            original_total,
            "total_allocated should be unchanged for reachable objects"
        );

        // The mark bit should be cleared after sweep.
        let header = unsafe { &*GcHeader::from_data_ptr(ptr) };
        assert!(
            !header.is_marked(),
            "mark bit should be cleared after sweep"
        );
    }

    #[test]
    fn test_collect_reduces_total_bytes() {
        // Allocate 10 objects, collect with empty roots. Total bytes should drop to 0.
        let mut heap = ActorHeap::new();
        for _ in 0..10 {
            heap.alloc(100, 8);
        }

        let before = heap.total_bytes();
        assert!(before > 0);

        let dummy: u64 = 0;
        let stack_ptr = &dummy as *const u64 as *const u8;
        heap.collect(stack_ptr, stack_ptr);

        assert_eq!(heap.total_bytes(), 0);
        assert!(
            heap.total_bytes() < before,
            "total_bytes should decrease after collection"
        );
    }

    #[test]
    fn test_gc_in_progress_guard() {
        // Verify gc_in_progress prevents re-entrant collection.
        let mut heap = ActorHeap::new();
        let _p = heap.alloc(64, 8);
        let before = heap.total_bytes();

        // Manually set gc_in_progress to true.
        heap.set_gc_in_progress(true);

        // Attempt collect -- should be a no-op due to re-entrancy guard.
        let dummy: u64 = 0;
        let stack_ptr = &dummy as *const u64 as *const u8;
        heap.collect(stack_ptr, stack_ptr);

        // Nothing should have changed.
        assert_eq!(
            heap.total_bytes(),
            before,
            "collect should be no-op when gc_in_progress is true"
        );
        assert!(
            !heap.all_objects_head().is_null(),
            "all_objects should be unchanged when gc_in_progress"
        );

        // gc_in_progress should still be true (collect was a no-op).
        assert!(heap.gc_in_progress());
    }

    #[test]
    fn test_collect_transitive_reachability() {
        // Object A (on fake stack) points to Object B. Both should survive.
        let mut heap = ActorHeap::new();

        // Allocate B first, then A. A's body will contain a pointer to B.
        let ptr_b = heap.alloc(64, 8);
        let ptr_a = heap.alloc(64, 8);

        // Write ptr_b into A's body so the mark phase traces A -> B.
        unsafe {
            *(ptr_a as *mut usize) = ptr_b as usize;
        }

        // Allocate a third object C that is NOT referenced.
        let _ptr_c = heap.alloc(64, 8);

        // Fake stack contains only ptr_a.
        let fake_stack: [usize; 4] = [0, ptr_a as usize, 0, 0];
        let stack_top = &fake_stack[0] as *const usize as *const u8;
        let stack_bottom = unsafe { stack_top.add(std::mem::size_of_val(&fake_stack)) };

        heap.collect(stack_bottom, stack_top);

        // A and B should survive, C should be freed.
        // Count surviving objects.
        let mut count = 0;
        let mut current = heap.all_objects_head();
        while !current.is_null() {
            count += 1;
            current = unsafe { (*current).next };
        }
        assert_eq!(
            count, 2,
            "A and B should survive (transitive reachability), C should be freed"
        );

        // total_allocated should reflect only A and B.
        assert_eq!(
            heap.total_bytes(),
            2 * (GC_HEADER_SIZE + 64),
            "total_bytes should reflect only the two surviving objects"
        );
    }

    #[test]
    fn test_find_object_containing_interior_pointer() {
        // A pointer into the middle of an object's body should identify that object.
        let mut heap = ActorHeap::new();
        let ptr = heap.alloc(128, 8);

        // Interior pointer: 64 bytes into the object.
        let interior = unsafe { ptr.add(64) };
        let found = heap.find_object_containing(interior);
        assert!(
            found.is_some(),
            "interior pointer should find the containing object"
        );

        let header = found.unwrap();
        let data_start = unsafe { (*header).data_ptr() };
        assert_eq!(
            data_start, ptr,
            "found object should be the one containing the interior pointer"
        );
    }

    #[test]
    fn test_find_object_containing_out_of_range() {
        let heap = ActorHeap::new();

        // Pointer outside any page should return None.
        let random_ptr = 0xDEADBEEF_usize as *const u8;
        assert!(heap.find_object_containing(random_ptr).is_none());
    }

    #[test]
    fn test_collect_then_reuse() {
        // After collection, freed objects should be reusable via the free list.
        let mut heap = ActorHeap::new();
        let _p1 = heap.alloc(64, 8);
        let _p2 = heap.alloc(64, 8);

        // Collect with empty roots to free everything.
        let dummy: u64 = 0;
        let stack_ptr = &dummy as *const u64 as *const u8;
        heap.collect(stack_ptr, stack_ptr);

        assert_eq!(heap.total_bytes(), 0);
        assert!(!heap.free_list_head().is_null());

        // Allocate again -- should reuse from free list.
        let p3 = heap.alloc(64, 8);
        assert!(!p3.is_null());
        // Should have come from the free list (total_allocated stays the same
        // because free-list reuse doesn't increment total_allocated in alloc).
        // Actually, the alloc_from_free_list doesn't add to total_allocated.
        // But p3 is now in all_objects.
        assert!(!heap.all_objects_head().is_null());
    }
}
