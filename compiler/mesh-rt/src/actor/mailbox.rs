//! FIFO mailbox for Mesh actor message passing.
//!
//! Each actor has a mailbox that delivers messages in strict FIFO order.
//! The mailbox is thread-safe (protected by a Mutex) since messages can
//! be sent from any actor on any worker thread.

use std::collections::VecDeque;

use parking_lot::Mutex;

use super::process::Message;

/// A thread-safe FIFO mailbox for an actor.
///
/// Messages are appended to the back (`push`) and removed from the front
/// (`pop`), ensuring strict FIFO delivery order. The internal queue is
/// protected by a `parking_lot::Mutex` for efficient cross-thread access.
pub struct Mailbox {
    queue: Mutex<VecDeque<Message>>,
}

impl Mailbox {
    /// Create a new empty mailbox.
    pub fn new() -> Self {
        Mailbox {
            queue: Mutex::new(VecDeque::new()),
        }
    }

    /// Append a message to the back of the mailbox (FIFO enqueue).
    pub fn push(&self, msg: Message) {
        self.queue.lock().push_back(msg);
    }

    /// Remove and return the front message (FIFO dequeue).
    ///
    /// Returns `None` if the mailbox is empty.
    pub fn pop(&self) -> Option<Message> {
        self.queue.lock().pop_front()
    }

    /// Check if the mailbox is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.lock().is_empty()
    }

    /// Return the number of messages in the mailbox.
    pub fn len(&self) -> usize {
        self.queue.lock().len()
    }

    /// Selectively remove the first message matching a predicate.
    ///
    /// Scans the mailbox from front to back and removes the first message
    /// for which `predicate` returns `true`. All other messages remain in
    /// their original order. Returns `None` if no message matches.
    ///
    /// This implements Erlang-style selective receive: the caller can wait
    /// for a specific message while leaving unrelated messages queued.
    pub fn remove_first<F>(&self, predicate: F) -> Option<Message>
    where
        F: Fn(&Message) -> bool,
    {
        let mut queue = self.queue.lock();
        for i in 0..queue.len() {
            if predicate(&queue[i]) {
                return queue.remove(i);
            }
        }
        None
    }
}

impl Default for Mailbox {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for Mailbox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.len();
        f.debug_struct("Mailbox").field("len", &len).finish()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actor::heap::MessageBuffer;

    fn make_msg(data: &[u8], tag: u64) -> Message {
        Message {
            buffer: MessageBuffer::new(data.to_vec(), tag),
        }
    }

    #[test]
    fn test_mailbox_push_pop_fifo() {
        let mb = Mailbox::new();

        mb.push(make_msg(&[1], 1));
        mb.push(make_msg(&[2], 2));
        mb.push(make_msg(&[3], 3));

        // Messages should come out in FIFO order.
        let m1 = mb.pop().unwrap();
        assert_eq!(m1.buffer.type_tag, 1);
        assert_eq!(m1.buffer.data, vec![1]);

        let m2 = mb.pop().unwrap();
        assert_eq!(m2.buffer.type_tag, 2);

        let m3 = mb.pop().unwrap();
        assert_eq!(m3.buffer.type_tag, 3);

        assert!(mb.pop().is_none());
    }

    #[test]
    fn test_mailbox_empty() {
        let mb = Mailbox::new();
        assert!(mb.is_empty());
        assert_eq!(mb.len(), 0);
        assert!(mb.pop().is_none());
    }

    #[test]
    fn test_mailbox_len() {
        let mb = Mailbox::new();
        assert_eq!(mb.len(), 0);

        mb.push(make_msg(&[1], 1));
        assert_eq!(mb.len(), 1);

        mb.push(make_msg(&[2], 2));
        assert_eq!(mb.len(), 2);

        mb.pop();
        assert_eq!(mb.len(), 1);
    }

    #[test]
    fn test_mailbox_concurrent_push() {
        use std::sync::Arc;

        let mb = Arc::new(Mailbox::new());
        let num_threads = 8;
        let msgs_per_thread = 100;

        let handles: Vec<_> = (0..num_threads)
            .map(|t| {
                let mb = Arc::clone(&mb);
                std::thread::spawn(move || {
                    for i in 0..msgs_per_thread {
                        let tag = (t * msgs_per_thread + i) as u64;
                        mb.push(make_msg(&[tag as u8], tag));
                    }
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(mb.len(), num_threads * msgs_per_thread);

        // Drain all messages -- should get exactly num_threads * msgs_per_thread.
        let mut count = 0;
        while mb.pop().is_some() {
            count += 1;
        }
        assert_eq!(count, num_threads * msgs_per_thread);
    }
}
