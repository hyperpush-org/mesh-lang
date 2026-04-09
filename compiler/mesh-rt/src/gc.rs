//! GC allocation entry points for Mesh runtime.
//!
//! Two allocation paths exist:
//!
//! - **Global arena** (`mesh_gc_alloc`): Simple bump allocator for non-actor
//!   contexts (main thread, startup). No object headers, no collection.
//!
//! - **Per-actor heap** (`mesh_gc_alloc_actor`): GcHeader-aware allocator that
//!   prepends a 16-byte header to every allocation. Supports mark-sweep GC
//!   with free-list reuse. Falls back to the global arena when no actor
//!   context is available.
//!
//! All GC-managed values (strings, closure environments, ADT payloads) are
//! allocated via these entry points. The returned pointer is always the
//! user-visible data pointer (past any header), keeping the ABI stable.

use std::sync::Mutex;

/// Default page size: 64 KiB.
const PAGE_SIZE: usize = 64 * 1024;

/// Global arena state protected by a mutex.
///
/// Single-threaded for Phase 5, but the mutex makes the API safe to call
/// from any context without UB concerns.
struct Arena {
    /// Allocated pages. Each page is a heap-allocated byte buffer.
    pages: Vec<Vec<u8>>,
    /// Offset into the current (last) page.
    offset: usize,
}

impl Arena {
    fn new() -> Self {
        Arena {
            pages: Vec::new(),
            offset: 0,
        }
    }

    fn init(&mut self) {
        if self.pages.is_empty() {
            self.pages.push(vec![0u8; PAGE_SIZE]);
            self.offset = 0;
        }
    }

    /// Bump-allocate `size` bytes with the given alignment.
    ///
    /// Returns a pointer to the allocated region. The pointer is valid for the
    /// lifetime of the arena (i.e., until the process exits in Phase 5).
    fn alloc(&mut self, size: usize, align: usize) -> *mut u8 {
        let align = if align == 0 { 1 } else { align };

        // Ensure the arena has been initialized.
        if self.pages.is_empty() {
            self.init();
        }

        // Align the current offset within the current page.
        let current_page_len = self.pages.last().map_or(0, |p| p.len());
        let aligned_offset = (self.offset + align - 1) & !(align - 1);

        if aligned_offset + size <= current_page_len {
            // Fits in the current page.
            let page = self.pages.last_mut().unwrap();
            let ptr = page[aligned_offset..].as_mut_ptr();
            self.offset = aligned_offset + size;
            ptr
        } else {
            // Allocate a new page. If the requested size exceeds the default
            // page size, allocate a page large enough to hold it.
            let new_page_size = if size > PAGE_SIZE {
                size + align
            } else {
                PAGE_SIZE
            };
            let mut page = vec![0u8; new_page_size];
            let aligned_start = {
                let base = page.as_ptr() as usize;
                let aligned = (base + align - 1) & !(align - 1);
                aligned - base
            };
            let ptr = page[aligned_start..].as_mut_ptr();
            self.offset = aligned_start + size;
            self.pages.push(page);
            ptr
        }
    }
}

static ARENA: Mutex<Option<Arena>> = Mutex::new(None);

/// Initialize the runtime arena. Called once at program start from `main`.
///
/// # Safety
///
/// This function is safe to call multiple times; subsequent calls are no-ops.
#[no_mangle]
pub extern "C" fn mesh_rt_init() {
    let mut guard = ARENA.lock().unwrap();
    if guard.is_none() {
        let mut arena = Arena::new();
        arena.init();
        *guard = Some(arena);
    }

    // Install ring crypto provider for TLS (PostgreSQL + ureq HTTP client).
    // Idempotent: ignore Err if already installed by another path.
    let _ = rustls::crypto::ring::default_provider().install_default();
}

/// Allocate `size` bytes with the given `align`ment from the GC arena.
///
/// Returns a pointer to zeroed memory. The pointer is valid for the lifetime
/// of the program (Phase 5 -- no collection).
///
/// # Safety
///
/// The returned pointer must not be freed by the caller. The arena owns
/// the memory.
#[no_mangle]
pub extern "C" fn mesh_gc_alloc(size: u64, align: u64) -> *mut u8 {
    let mut guard = ARENA.lock().unwrap();
    let arena = guard.get_or_insert_with(|| {
        let mut a = Arena::new();
        a.init();
        a
    });
    arena.alloc(size as usize, align as usize)
}

/// Allocate `size` bytes with the given `align`ment from the current actor's
/// per-actor heap.
///
/// If called from within an actor context (i.e., a thread running an actor
/// coroutine), allocates from that actor's heap via `ActorHeap::alloc()`,
/// which prepends a 16-byte `GcHeader`. The returned pointer is past the
/// header -- callers see only user data.
///
/// Falls back to the global arena (no header) if no actor context is available.
///
/// # Safety
///
/// The returned pointer must not be freed by the caller. The actor's heap
/// owns the memory.
#[no_mangle]
pub extern "C" fn mesh_gc_alloc_actor(size: u64, align: u64) -> *mut u8 {
    // Try to allocate from the current actor's heap.
    if let Some(ptr) = try_alloc_from_actor_heap(size as usize, align as usize) {
        return ptr;
    }
    // Fallback to global arena.
    mesh_gc_alloc(size, align)
}

/// Attempt to allocate from the current actor's per-actor heap.
///
/// Returns `Some(ptr)` if running in an actor context and the allocation
/// succeeded. Returns `None` if no actor context is available.
fn try_alloc_from_actor_heap(size: usize, align: usize) -> Option<*mut u8> {
    use crate::actor::stack::get_current_pid;

    let pid = get_current_pid()?;

    // Access the global scheduler's process table to find this actor's heap.
    use crate::actor::GLOBAL_SCHEDULER;
    let sched = GLOBAL_SCHEDULER.get()?;
    let proc_arc = sched.get_process(pid)?;
    let mut proc = proc_arc.lock();
    Some(proc.heap.alloc(size, align))
}

/// Trigger garbage collection on the current actor's heap.
///
/// Explicitly forces a mark-sweep GC cycle on the calling actor's heap,
/// regardless of heap pressure. This can be called from Mesh code via
/// `System.gc()` or similar intrinsic.
///
/// The function:
/// 1. Conservatively scans the actor's coroutine stack for roots
/// 2. Marks all transitively reachable objects
/// 3. Sweeps unmarked objects onto the free list for reuse
///
/// No-op if called outside of an actor context or if GC is already in
/// progress (re-entrancy guard).
#[no_mangle]
pub extern "C" fn mesh_gc_collect() {
    use crate::actor::stack;
    use crate::actor::GLOBAL_SCHEDULER;

    let pid = match stack::get_current_pid() {
        Some(pid) => pid,
        None => return,
    };

    let sched = match GLOBAL_SCHEDULER.get() {
        Some(s) => s,
        None => return,
    };

    let proc_arc = match sched.get_process(pid) {
        Some(p) => p,
        None => return,
    };

    // Capture current stack position as stack_top.
    let stack_anchor: u64 = 0;
    let _ = std::hint::black_box(&stack_anchor);
    let stack_top = &stack_anchor as *const u64 as *const u8;

    let mut proc = proc_arc.lock();

    // Read stack_base from the process object rather than the STACK_BASE
    // thread-local. The thread-local may be stale if another coroutine ran
    // on this thread and overwrote it. The process field is set once at
    // coroutine startup and never changes.
    let stack_bottom = proc.stack_base;
    if stack_bottom.is_null() {
        return;
    }

    proc.heap.collect(stack_bottom, stack_top);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arena_basic_alloc() {
        let mut arena = Arena::new();
        arena.init();

        let ptr1 = arena.alloc(16, 8);
        assert!(!ptr1.is_null());

        let ptr2 = arena.alloc(32, 8);
        assert!(!ptr2.is_null());

        // Pointers should be different.
        assert_ne!(ptr1, ptr2);
    }

    #[test]
    fn test_arena_alignment() {
        let mut arena = Arena::new();
        arena.init();

        let ptr = arena.alloc(8, 16);
        assert!(!ptr.is_null());
        assert_eq!(ptr as usize % 16, 0, "pointer should be 16-byte aligned");
    }

    #[test]
    fn test_arena_large_alloc() {
        let mut arena = Arena::new();
        arena.init();

        // Allocate more than a page.
        let ptr = arena.alloc(128 * 1024, 8);
        assert!(!ptr.is_null());
    }

    #[test]
    fn test_gc_alloc_extern() {
        mesh_rt_init();
        let ptr = mesh_gc_alloc(64, 8);
        assert!(!ptr.is_null());
    }

    #[test]
    fn test_mesh_gc_collect_no_crash_outside_actor() {
        // When called outside an actor context (no current PID), should be a
        // no-op without crashing.
        mesh_gc_collect();
    }

    #[test]
    fn test_global_arena_no_headers() {
        // Global arena does NOT prepend GcHeaders -- it returns raw pointers.
        let mut arena = Arena::new();
        arena.init();

        // Write a known pattern to the arena allocation.
        let ptr = arena.alloc(8, 8);
        unsafe {
            std::ptr::write(ptr as *mut u64, 0xDEADBEEF_CAFEBABE);
        }

        // Read it back -- the pointer IS the data, no header to skip.
        let val = unsafe { *(ptr as *const u64) };
        assert_eq!(val, 0xDEADBEEF_CAFEBABE);
    }

    #[test]
    fn test_actor_heap_has_headers() {
        use crate::actor::heap::{ActorHeap, GcHeader, GC_HEADER_SIZE};

        // Actor heap DOES prepend GcHeaders to every allocation.
        let mut heap = ActorHeap::new();
        let data_ptr = heap.alloc(64, 8);

        // Back up 16 bytes to find the GcHeader.
        let header_ptr = unsafe { GcHeader::from_data_ptr(data_ptr) };
        let header = unsafe { &*header_ptr };

        // The header should have the correct size and be alive (not free).
        assert_eq!(header.size, 64);
        assert!(!header.is_free());
        assert!(!header.is_marked());

        // Verify the data pointer is exactly GC_HEADER_SIZE bytes past the header.
        assert_eq!(
            data_ptr as usize - header_ptr as usize,
            GC_HEADER_SIZE,
            "data pointer should be GC_HEADER_SIZE bytes past the header"
        );
    }
}
