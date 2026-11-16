//! Looks for git repos on disk that could become projects.

use std::fs;
use std::path::{Path, PathBuf};

/// How deep below a search folder a repo can be: `~/Documents/work/client/app` is 3.
pub const MAX_DEPTH: usize = 3;

/// Folders that hold dependencies or build output, never a repo worth adding, and can
/// be huge.
const SKIPPED: [&str; 10] = [
    "node_modules",
    "target",
    "vendor",
    "dist",
    "build",
    "Pods",
    "DerivedData",
    "Library",
    "Applications",
    "venv",
];

/// The repos under `folders`, at most `MAX_DEPTH` levels down, sorted and leaving out
/// `known`. Paths are canonical. Folders that do not exist are skipped.
pub fn find_repos(folders: &[PathBuf], known: &[PathBuf]) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for folder in folders {
        walk(folder, 0, &mut found);
    }
    found.sort();
    found.dedup();
    found.retain(|repo| !known.contains(repo));
    found
}

fn walk(dir: &Path, depth: usize, found: &mut Vec<PathBuf>) {
    // Only a `.git` folder: a `.git` file belongs to a worktree, like a treehouse slot,
    // or to a submodule.
    if dir.join(".git").is_dir() {
        if let Ok(repo) = dir.canonicalize() {
            found.push(repo);
        }
        return;
    }
    if depth == MAX_DEPTH {
        return;
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.starts_with('.') || SKIPPED.contains(&name.as_ref()) {
            continue;
        }
        // `file_type` does not follow symlinks, so a link loop cannot trap the search.
        if entry.file_type().is_ok_and(|kind| kind.is_dir()) {
            walk(&entry.path(), depth + 1, found);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fresh folder under the system temp folder, removed on drop.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(name: &str) -> Self {
            let dir = std::env::temp_dir().join(format!("f0-scan-{name}-{}", std::process::id()));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            Self(dir.canonicalize().unwrap())
        }

        fn repo(&self, rel: &str) -> PathBuf {
            let path = self.0.join(rel);
            fs::create_dir_all(path.join(".git")).unwrap();
            path
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn finds_repos_down_to_the_max_depth() {
        let tmp = TempDir::new("depth");
        let shallow = tmp.repo("app");
        let deep = tmp.repo("work/client/site");
        tmp.repo("a/b/c/too-deep");
        assert_eq!(
            find_repos(std::slice::from_ref(&tmp.0), &[]),
            [shallow, deep]
        );
    }

    #[test]
    fn skips_dependencies_hidden_folders_worktrees_and_nested_repos() {
        let tmp = TempDir::new("skip");
        let app = tmp.repo("app");
        tmp.repo("app/packages/inner");
        tmp.repo("node_modules/dep");
        tmp.repo(".cache/thing");
        tmp.repo("x/target/gen");
        let slot = tmp.0.join("pool/1/app");
        fs::create_dir_all(&slot).unwrap();
        fs::write(slot.join(".git"), "gitdir: /elsewhere").unwrap();
        assert_eq!(find_repos(std::slice::from_ref(&tmp.0), &[]), [app]);
    }

    #[test]
    fn leaves_out_known_repos_repeats_and_missing_folders() {
        let tmp = TempDir::new("known");
        let known = tmp.repo("known");
        let new = tmp.repo("new");
        let folders = [tmp.0.clone(), tmp.0.clone(), tmp.0.join("missing")];
        assert_eq!(find_repos(&folders, &[known]), [new]);
    }
}
