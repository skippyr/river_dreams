use std::{fs, path::PathBuf};

const DEFAULT_BRANCH_NAME: &str = "master";

#[derive(Debug, Clone)]
pub(crate) enum Reference {
    Branch(String),
    RebaseHash(String),
}

#[derive(Debug, Clone)]
pub(crate) struct Repository {
    pub(crate) path: PathBuf,
    pub(crate) reference: Reference,
    pub(crate) is_dirty: bool,
}

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
