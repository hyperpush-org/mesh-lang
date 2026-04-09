//! GC-managed immutable FIFO Queue for the Mesh runtime.
//!
//! A MeshQueue is backed by two lists (front + back) for amortized O(1)
//! push/pop. Layout is an opaque GC-allocated struct:
//! `{ u64 front_ptr, u64 back_ptr }`.
//!
//! All operations return NEW queues (immutable semantics).

use crate::gc::mesh_gc_alloc_actor;

// ── Internal helpers ──────────────────────────────────────────────────

/// Queue layout: { front: *mut u8 (list), back: *mut u8 (list) }
const QUEUE_SIZE: usize = 16; // 2 pointers as u64

unsafe fn queue_front(q: *const u8) -> *mut u8 {
    *(q as *const u64) as *mut u8
}

unsafe fn queue_back(q: *const u8) -> *mut u8 {
    *((q as *const u64).add(1)) as *mut u8
}

unsafe fn alloc_queue(front: *mut u8, back: *mut u8) -> *mut u8 {
    let p = mesh_gc_alloc_actor(QUEUE_SIZE as u64, 8);
    *(p as *mut u64) = front as u64;
    *((p as *mut u64).add(1)) = back as u64;
    p
}

/// Transfer the back list to the front when front is empty.
///
/// Because our back list is built with `append` (chronological order),
/// we transfer it directly (no reversal needed) to maintain FIFO order.
unsafe fn normalize(front: *mut u8, back: *mut u8) -> (*mut u8, *mut u8) {
    use super::list;
    if list::mesh_list_length(front) == 0 && list::mesh_list_length(back) > 0 {
        (back, list::mesh_list_new())
    } else {
        (front, back)
    }
}

// ── Public API ────────────────────────────────────────────────────────

/// Create an empty queue.
#[no_mangle]
pub extern "C" fn mesh_queue_new() -> *mut u8 {
    unsafe {
        let front = super::list::mesh_list_new();
        let back = super::list::mesh_list_new();
        alloc_queue(front, back)
    }
}

/// Push an element to the back of the queue. Returns a NEW queue.
#[no_mangle]
pub extern "C" fn mesh_queue_push(queue: *mut u8, element: u64) -> *mut u8 {
    unsafe {
        let front = queue_front(queue);
        let new_back = super::list::mesh_list_append(queue_back(queue), element);
        let (f, b) = normalize(front, new_back);
        alloc_queue(f, b)
    }
}

/// Pop an element from the front. Returns a tuple-like struct:
/// `{ u64 element, u64 new_queue_ptr }` (16 bytes, GC-allocated).
///
/// Panics if the queue is empty.
#[no_mangle]
pub extern "C" fn mesh_queue_pop(queue: *mut u8) -> *mut u8 {
    unsafe {
        let front = queue_front(queue);
        let back = queue_back(queue);

        // Normalize if needed.
        let (front, back) = normalize(front, back);

        if super::list::mesh_list_length(front) == 0 {
            panic!("mesh_queue_pop: empty queue");
        }

        let element = super::list::mesh_list_head(front);
        let new_front = super::list::mesh_list_tail(front);
        let (nf, nb) = normalize(new_front, back);
        let new_queue = alloc_queue(nf, nb);

        // Return a pair: { element, new_queue_ptr }.
        let result = mesh_gc_alloc_actor(16, 8);
        *(result as *mut u64) = element;
        *((result as *mut u64).add(1)) = new_queue as u64;
        result
    }
}

/// Peek at the front element without removing it. Panics if empty.
#[no_mangle]
pub extern "C" fn mesh_queue_peek(queue: *mut u8) -> u64 {
    unsafe {
        let front = queue_front(queue);
        let back = queue_back(queue);
        let (front, _) = normalize(front, back);

        if super::list::mesh_list_length(front) == 0 {
            panic!("mesh_queue_peek: empty queue");
        }

        super::list::mesh_list_head(front)
    }
}

/// Return the total number of elements in the queue.
#[no_mangle]
pub extern "C" fn mesh_queue_size(queue: *mut u8) -> i64 {
    unsafe {
        let front_len = super::list::mesh_list_length(queue_front(queue));
        let back_len = super::list::mesh_list_length(queue_back(queue));
        front_len + back_len
    }
}

/// Returns 1 if the queue is empty, 0 otherwise.
#[no_mangle]
pub extern "C" fn mesh_queue_is_empty(queue: *mut u8) -> i8 {
    if mesh_queue_size(queue) == 0 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gc::mesh_rt_init;

    #[test]
    fn test_queue_new_is_empty() {
        mesh_rt_init();
        let q = mesh_queue_new();
        assert_eq!(mesh_queue_size(q), 0);
        assert_eq!(mesh_queue_is_empty(q), 1);
    }

    #[test]
    fn test_queue_push_pop_fifo() {
        mesh_rt_init();
        let q = mesh_queue_new();
        let q = mesh_queue_push(q, 10);
        let q = mesh_queue_push(q, 20);
        let q = mesh_queue_push(q, 30);
        assert_eq!(mesh_queue_size(q), 3);

        // Pop should return elements in FIFO order.
        let result = mesh_queue_pop(q);
        unsafe {
            let elem = *(result as *const u64);
            let new_q = *((result as *const u64).add(1)) as *mut u8;
            assert_eq!(elem, 10);
            assert_eq!(mesh_queue_size(new_q), 2);

            let result2 = mesh_queue_pop(new_q);
            let elem2 = *(result2 as *const u64);
            assert_eq!(elem2, 20);
        }
    }

    #[test]
    fn test_queue_peek() {
        mesh_rt_init();
        let q = mesh_queue_new();
        let q = mesh_queue_push(q, 42);
        let q = mesh_queue_push(q, 99);
        assert_eq!(mesh_queue_peek(q), 42);
        // Peek doesn't remove the element.
        assert_eq!(mesh_queue_size(q), 2);
    }

    #[test]
    fn test_queue_immutability() {
        mesh_rt_init();
        let q1 = mesh_queue_new();
        let q2 = mesh_queue_push(q1, 1);
        assert_eq!(mesh_queue_size(q1), 0);
        assert_eq!(mesh_queue_size(q2), 1);
    }

    #[test]
    fn test_queue_is_empty_after_pop_all() {
        mesh_rt_init();
        let q = mesh_queue_new();
        let q = mesh_queue_push(q, 1);
        let result = mesh_queue_pop(q);
        unsafe {
            let new_q = *((result as *const u64).add(1)) as *mut u8;
            assert_eq!(mesh_queue_is_empty(new_q), 1);
        }
    }
}
