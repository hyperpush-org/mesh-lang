pub mod lockfile;
pub mod manifest;
pub mod resolver;
pub mod scaffold;
pub mod toolchain_update;

// Re-export key types for convenience.
pub use lockfile::{LockedPackage, Lockfile};
pub use manifest::{
    build_clustered_export_surface, collect_source_cluster_declarations,
    validate_cluster_declarations, validate_cluster_declarations_with_source, ClusterConfig,
    ClusteredDeclaration, ClusteredDeclarationError, ClusteredDeclarationKind,
    ClusteredDeclarationOrigin, ClusteredDeclarationProvenance, ClusteredExecutableSurfaceInfo,
    ClusteredExecutionMetadata, ClusteredExportSurface, ClusteredReplicationCount,
    ClusteredReplicationCountSource, Dependency, Manifest, Package, SourceClusteredDeclaration,
    SourceClusteredDeclarationSyntax, DEFAULT_CLUSTER_REPLICATION_COUNT,
};
pub use resolver::resolve_dependencies;
pub use scaffold::{
    scaffold_clustered_project, scaffold_project, scaffold_todo_api_project,
    scaffold_todo_api_project_with_db, TodoApiDatabase,
};
pub use toolchain_update::{
    run_toolchain_update, ToolchainUpdateError, ToolchainUpdateMode, ToolchainUpdateOutcome,
};
