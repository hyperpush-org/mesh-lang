//! Named process registration and lookup.
//!
//! Actors can register themselves under a string name and be looked up by
//! other actors using that name. This enables service discovery without
//! passing PIDs explicitly.
//!
//! ## Semantics
//!
//! - A name can only be registered to one process at a time.
//! - Registering an already-taken name returns an error.
//! - When a process exits, all its registered names are automatically cleaned up.
//! - `whereis(name)` returns the PID for a registered name, or None.

use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use std::sync::OnceLock;

use super::process::ProcessId;

// ---------------------------------------------------------------------------
// ProcessRegistry
// ---------------------------------------------------------------------------

/// Global named process registry.
///
/// Maps string names to PIDs. Protected by an RwLock for concurrent read
/// access with exclusive write access.
pub struct ProcessRegistry {
    /// name -> PID mapping
    names: RwLock<FxHashMap<String, ProcessId>>,
    /// PID -> names reverse index for efficient cleanup on process exit
    pid_names: RwLock<FxHashMap<ProcessId, Vec<String>>>,
}

/// Error returned when a name is already registered.
#[derive(Debug)]
pub struct NameAlreadyRegistered {
    pub name: String,
    pub existing_pid: ProcessId,
}

impl std::fmt::Display for NameAlreadyRegistered {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name '{}' already registered to {}",
            self.name, self.existing_pid
        )
    }
}

impl ProcessRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        ProcessRegistry {
            names: RwLock::new(FxHashMap::default()),
            pid_names: RwLock::new(FxHashMap::default()),
        }
    }

    /// Register a process under a name.
    ///
    /// Returns `Ok(())` if the name was successfully registered, or
    /// `Err(NameAlreadyRegistered)` if the name is already taken.
    pub fn register(&self, name: String, pid: ProcessId) -> Result<(), NameAlreadyRegistered> {
        let mut names = self.names.write();

        if let Some(&existing_pid) = names.get(&name) {
            return Err(NameAlreadyRegistered { name, existing_pid });
        }

        names.insert(name.clone(), pid);

        // Update reverse index.
        self.pid_names.write().entry(pid).or_default().push(name);

        Ok(())
    }

    /// Look up a process by name.
    ///
    /// Returns `Some(pid)` if the name is registered, `None` otherwise.
    pub fn whereis(&self, name: &str) -> Option<ProcessId> {
        self.names.read().get(name).copied()
    }

    /// Unregister a name.
    ///
    /// Returns `true` if the name was found and removed, `false` if not found.
    pub fn unregister(&self, name: &str) -> bool {
        let mut names = self.names.write();
        if let Some(pid) = names.remove(name) {
            // Remove from reverse index.
            let mut pid_names = self.pid_names.write();
            if let Some(name_list) = pid_names.get_mut(&pid) {
                name_list.retain(|n| n != name);
                if name_list.is_empty() {
                    pid_names.remove(&pid);
                }
            }
            true
        } else {
            false
        }
    }

    /// Remove all registrations for a process.
    ///
    /// Called during process exit cleanup. Removes all names registered
    /// to the given PID from both the name->PID and PID->names maps.
    pub fn cleanup_process(&self, pid: ProcessId) {
        // Get all names for this PID.
        let names_to_remove = {
            let mut pid_names = self.pid_names.write();
            pid_names.remove(&pid).unwrap_or_default()
        };

        // Remove each name from the name->PID map.
        if !names_to_remove.is_empty() {
            let mut names = self.names.write();
            for name in &names_to_remove {
                names.remove(name);
            }
        }
    }
}

impl Default for ProcessRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Global registry instance
// ---------------------------------------------------------------------------

/// The global process registry, lazily initialized.
static GLOBAL_REGISTRY: OnceLock<ProcessRegistry> = OnceLock::new();

/// Get a reference to the global process registry.
pub fn global_registry() -> &'static ProcessRegistry {
    GLOBAL_REGISTRY.get_or_init(ProcessRegistry::new)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Create a fresh registry for testing (avoids global state interference).
    fn fresh_registry() -> ProcessRegistry {
        ProcessRegistry::new()
    }

    #[test]
    fn test_register_and_whereis() {
        let reg = fresh_registry();
        let pid = ProcessId::next();

        reg.register("my_server".to_string(), pid).unwrap();

        assert_eq!(reg.whereis("my_server"), Some(pid));
        assert_eq!(reg.whereis("nonexistent"), None);
    }

    #[test]
    fn test_register_duplicate_name_fails() {
        let reg = fresh_registry();
        let pid1 = ProcessId::next();
        let pid2 = ProcessId::next();

        reg.register("server".to_string(), pid1).unwrap();
        let err = reg.register("server".to_string(), pid2).unwrap_err();

        assert_eq!(err.name, "server");
        assert_eq!(err.existing_pid, pid1);
    }

    #[test]
    fn test_unregister() {
        let reg = fresh_registry();
        let pid = ProcessId::next();

        reg.register("temp".to_string(), pid).unwrap();
        assert!(reg.whereis("temp").is_some());

        let removed = reg.unregister("temp");
        assert!(removed);
        assert_eq!(reg.whereis("temp"), None);

        // Unregistering again returns false.
        let removed_again = reg.unregister("temp");
        assert!(!removed_again);
    }

    #[test]
    fn test_cleanup_process_removes_all_names() {
        let reg = fresh_registry();
        let pid = ProcessId::next();

        reg.register("name1".to_string(), pid).unwrap();
        reg.register("name2".to_string(), pid).unwrap();
        reg.register("name3".to_string(), pid).unwrap();

        assert!(reg.whereis("name1").is_some());
        assert!(reg.whereis("name2").is_some());
        assert!(reg.whereis("name3").is_some());

        reg.cleanup_process(pid);

        assert_eq!(reg.whereis("name1"), None);
        assert_eq!(reg.whereis("name2"), None);
        assert_eq!(reg.whereis("name3"), None);
    }

    #[test]
    fn test_cleanup_nonexistent_process_is_noop() {
        let reg = fresh_registry();
        let pid = ProcessId::next();
        // Should not panic.
        reg.cleanup_process(pid);
    }

    #[test]
    fn test_register_after_cleanup_succeeds() {
        let reg = fresh_registry();
        let pid1 = ProcessId::next();
        let pid2 = ProcessId::next();

        reg.register("server".to_string(), pid1).unwrap();
        reg.cleanup_process(pid1);

        // Name should now be available for re-registration.
        reg.register("server".to_string(), pid2).unwrap();
        assert_eq!(reg.whereis("server"), Some(pid2));
    }

    #[test]
    fn test_multiple_processes_different_names() {
        let reg = fresh_registry();
        let pid1 = ProcessId::next();
        let pid2 = ProcessId::next();

        reg.register("server_a".to_string(), pid1).unwrap();
        reg.register("server_b".to_string(), pid2).unwrap();

        assert_eq!(reg.whereis("server_a"), Some(pid1));
        assert_eq!(reg.whereis("server_b"), Some(pid2));

        // Cleanup pid1 should only remove server_a.
        reg.cleanup_process(pid1);
        assert_eq!(reg.whereis("server_a"), None);
        assert_eq!(reg.whereis("server_b"), Some(pid2));
    }

    #[test]
    fn test_concurrent_register_whereis() {
        use std::sync::Arc;

        let reg = Arc::new(fresh_registry());
        let num_threads = 8;

        let handles: Vec<_> = (0..num_threads)
            .map(|t| {
                let reg = Arc::clone(&reg);
                std::thread::spawn(move || {
                    let pid = ProcessId::next();
                    let name = format!("worker_{}", t);
                    reg.register(name.clone(), pid).unwrap();
                    // Verify our own registration.
                    assert_eq!(reg.whereis(&name), Some(pid));
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }

        // All workers should be registered.
        for t in 0..num_threads {
            let name = format!("worker_{}", t);
            assert!(
                reg.whereis(&name).is_some(),
                "worker_{} should be registered",
                t
            );
        }
    }
}
