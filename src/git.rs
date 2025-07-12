//! Provides features to retrieve Git repositories metadata.

use std::fs;
use std::path::PathBuf;

/// The default branch name used by Git.
const DEFAULT_BRANCH_NAME: &str = "master";

/// Contains the possible references types used by Git to identify the state of a repository.
#[derive(Debug, Clone)]
pub(crate) enum Reference {
    /// A branch name.
    Branch(String),
    /// A hash describing a rebase operation.
    RebaseHash(String),
}

/// Contains the metadata of a Git repository.
#[derive(Debug, Clone)]
pub(crate) struct Repository {
    /// The full path to the repository.
    pub(crate) path: PathBuf,
    /// The type of reference being used to represent its current state.
    pub(crate) reference: Reference,
    /// A boolean that states it is dirty, this is, it contains uncommited changes.
    pub(crate) is_dirty: bool,
}

/// Finds the metadata of a possibly active Git repository by searching recursively from the current
/// directory.
///
/// # Returns
/// The possible repository fimd.
pub(crate) fn find_repository() -> Option<Repository> {
    let repository = git2::Repository::discover(".").ok()?;
    let reference = repository
        .head()
        .ok()
        .and_then(|head| {
            if repository.state() == git2::RepositoryState::RebaseInteractive {
                head.target()
                    .map(|hash| Reference::RebaseHash(hash.to_string().chars().take(7).collect()))
            } else {
                head.shorthand()
                    .map(|branch| Reference::Branch(branch.to_string()))
            }
        })
        .or_else(|| {
            fs::read_to_string(repository.path().join("HEAD"))
                .ok()
                .and_then(|head_data| {
                    head_data
                        .trim()
                        .strip_prefix("ref: refs/heads/")
                        .map(|branch| Reference::Branch(branch.to_string()))
                })
        })
        .or_else(|| {
            repository
                .config()
                .ok()
                .and_then(|config| config.get_string("init.defaultBranch").ok())
                .map(Reference::Branch)
        })
        .unwrap_or_else(|| Reference::Branch(DEFAULT_BRANCH_NAME.to_string()));
    let is_dirty = repository
        .statuses(None)
        .ok()
        .map(|entries| {
            entries
                .iter()
                .filter(|entry| !entry.status().contains(git2::Status::IGNORED))
                .any(|entry| {
                    entry.status().intersects(
                        git2::Status::INDEX_NEW
                            | git2::Status::INDEX_MODIFIED
                            | git2::Status::INDEX_DELETED
                            | git2::Status::INDEX_RENAMED
                            | git2::Status::INDEX_TYPECHANGE
                            | git2::Status::WT_NEW
                            | git2::Status::WT_MODIFIED
                            | git2::Status::WT_DELETED
                            | git2::Status::WT_TYPECHANGE
                            | git2::Status::WT_RENAMED,
                    )
                })
        })
        .unwrap_or(false);
    Some(Repository {
        path: repository
            .workdir()
            .unwrap_or_else(|| repository.path())
            .to_path_buf(),
        reference,
        is_dirty,
    })
}
