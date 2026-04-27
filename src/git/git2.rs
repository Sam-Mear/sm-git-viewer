// Thin adapter implementing the interfaces in interfaces.rs using the `git2` crate.

use crate::git::interfaces::{Repository as RepoTrait, DataProvider as DataProviderTrait, Workflow as WorkflowTrait};
use crate::config::Config;

pub struct Git2Repository {
    repo: git2::Repository,
    name: String,
    current_branch: String,
}

impl Git2Repository {
    pub fn open<P: AsRef<std::path::Path>>(path: P) -> Result<Box<dyn RepoTrait>, git2::Error> {
        let repo = git2::Repository::open(path.as_ref())?;
        // derive a name and current branch snapshot
        let name = repo
            .path()
            .to_str()
            .map(|s| s.to_string())
            .unwrap_or_default();

        let current_branch = repo
            .head()
            .ok()
            .and_then(|h| h.shorthand().map(|s| s.to_string()))
            .unwrap_or_else(|| "HEAD".to_string());

        Ok(Box::new(Git2Repository {
            repo,
            name,
            current_branch,
        }))
    }
}

impl RepoTrait for Git2Repository {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_current_branch(&self) -> &str {
        &self.current_branch
    }

    fn get_branches(&self) -> Vec<String> {
        match self.repo.branches(None) {
            Ok(branches) => branches
                .filter_map(|b| b.ok())
                .filter_map(|(branch, _)| {
                    branch
                        .name()
                        .ok()
                        .and_then(|opt| opt.map(|s| s.to_string()))
                })
                .collect(),
            Err(_) => vec![],
        }
    }

    fn get_remotes(&self) -> Vec<String> {
        match self.repo.remotes() {
            Ok(list) => list.iter().filter_map(|r| r.map(|s| s.to_string())).collect(),
            Err(_) => vec![],
        }
    }

    fn get_path(&self) -> &str {
        // git2::Repository::path returns the path to the .git directory
        self.repo.path().to_str().unwrap_or_default()
    }

    fn get_remote_urls(&self) -> Vec<String> {
        match self.repo.remotes() {
            Ok(list) => {
                let mut urls = Vec::new();
                for maybe_name in list.iter() {
                    if let Some(name) = maybe_name {
                        if let Ok(remote) = self.repo.find_remote(name) {
                            if let Some(url) = remote.url() {
                                urls.push(url.to_string());
                            }
                        }
                    }
                }
                urls
            }
            Err(_) => vec![],
        }
    }
}

// A minimal fallback repository used when opening a real repo fails.
struct EmptyRepository {
    path: String,
}

impl EmptyRepository {
    fn new(path: &str) -> Self {
        Self { path: path.to_string() }
    }
}

impl RepoTrait for EmptyRepository {
    fn get_name(&self) -> &str {
        &self.path
    }

    fn get_current_branch(&self) -> &str {
        "unknown"
    }

    fn get_branches(&self) -> Vec<String> {
        vec![]
    }

    fn get_remotes(&self) -> Vec<String> {
        vec![]
    }

    fn get_path(&self) -> &str {
        &self.path
    }

    fn get_remote_urls(&self) -> Vec<String> {
        vec![]
    }
}

// A simple workflow stub
struct EmptyWorkflow;
impl WorkflowTrait for EmptyWorkflow {
    fn get_issues(&self) -> Vec<String> {
        vec![]
    }
}

/// A DataProvider implementation that constructs a repository from `Config`.
pub struct ConfigDataProvider {
    repo_path: Option<String>,
}

impl ConfigDataProvider {
    pub fn new(cfg: &Config) -> Self {
        Self {
            repo_path: cfg.repo_path.clone(),
        }
    }

    // Provide an inherent method so callers don't need to import the trait.
    pub fn get_repository(&self) -> Box<dyn RepoTrait> {
        let path = self.repo_path.as_deref().unwrap_or(".");
        match Git2Repository::open(path) {
            Ok(repo) => repo,
            Err(_) => Box::new(EmptyRepository::new(path)),
        }
    }
}

impl DataProviderTrait for ConfigDataProvider {
    fn get_repository(&self) -> Box<dyn RepoTrait> {
        let path = self
            .repo_path
            .as_deref()
            .unwrap_or(".");

        match Git2Repository::open(path) {
            Ok(repo) => repo,
            Err(_) => Box::new(EmptyRepository::new(path)),
        }
    }

    fn get_workflow(&self) -> Box<dyn WorkflowTrait> {
        Box::new(EmptyWorkflow)
    }
}
