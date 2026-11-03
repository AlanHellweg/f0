//! What keeps the bundled theme from applying: a tmux without theme options, and colors
//! set in other Ghostty config files, which win over the bundled `theme`.

use std::path::{Path, PathBuf};

/// The first tmux with `theme`, `dark-theme-*` and `light-theme-*`.
pub const MIN_TMUX: (u32, u32) = (3, 8);

const GHOSTTY_COLOR_KEYS: [&str; 7] = [
    "background",
    "foreground",
    "palette",
    "cursor-color",
    "cursor-text",
    "selection-background",
    "selection-foreground",
];

/// Whether `link` is a symlink to `<repo>/<rel>` inside an f0 repo, as install.sh leaves it.
pub fn is_bundled(link: &Path, rel: &Path) -> bool {
    let Ok(target) = std::fs::read_link(link) else {
        return false;
    };
    let target = match link.parent() {
        Some(dir) if target.is_relative() => dir.join(target),
        _ => target,
    };
    let mut repo = target.as_path();
    for _ in rel.components() {
        match repo.parent() {
            Some(parent) => repo = parent,
            None => return false,
        }
    }
    repo.join(rel) == target && repo.join("crates/f0").is_dir()
}

/// Parses `tmux -V`: `tmux 3.8`, `tmux 3.5a` or `tmux next-3.8`.
pub fn tmux_version(out: &str) -> Option<(u32, u32)> {
    let version = out.trim().strip_prefix("tmux ")?;
    let version = version.strip_prefix("next-").unwrap_or(version);
    let (major, rest) = version.split_once('.')?;
    let minor_end = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    Some((major.parse().ok()?, rest[..minor_end].parse().ok()?))
}

/// Ghostty config files other than the bundled one, in the order Ghostty loads them.
pub fn other_ghostty_files(home: &Path, xdg: &Path) -> Vec<PathBuf> {
    let app_support = home.join("Library/Application Support/com.mitchellh.ghostty");
    vec![
        xdg.join("ghostty/config.ghostty"),
        app_support.join("config"),
        app_support.join("config.ghostty"),
    ]
}

/// The lines of a Ghostty config that set a color the theme would otherwise set, as
/// 1-based line numbers and keys. An empty value resets the key, so it is not a color.
pub fn color_overrides(text: &str) -> Vec<(usize, &str)> {
    text.lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let line = line.trim();
            if line.starts_with('#') {
                return None;
            }
            let (key, value) = line.split_once('=')?;
            let key = key.trim();
            let is_color = GHOSTTY_COLOR_KEYS.contains(&key) && !value.trim().is_empty();
            is_color.then_some((index + 1, key))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_color_lines_with_or_without_spaces() {
        let text = "\
font-size = 14
background = #ffffff
foreground=#000000
  cursor-color =  #333333
palette = 0=#1e1e1e
palette=15=#ffffff
";
        assert_eq!(
            color_overrides(text),
            vec![
                (2, "background"),
                (3, "foreground"),
                (4, "cursor-color"),
                (5, "palette"),
                (6, "palette"),
            ]
        );
    }

    #[test]
    fn skips_comments_resets_blank_lines_and_similar_keys() {
        let text = "\
# background = #ffffff
  # foreground = #000000

background =
background-opacity = 0.9
selection-foreground-extra = #fff
theme = dark:Catppuccin Mocha
";
        assert!(color_overrides(text).is_empty());
    }

    #[test]
    fn parses_tmux_versions() {
        assert_eq!(tmux_version("tmux 3.8\n"), Some((3, 8)));
        assert_eq!(tmux_version("tmux 3.5a"), Some((3, 5)));
        assert_eq!(tmux_version("tmux next-3.8"), Some((3, 8)));
        assert_eq!(tmux_version("tmux 10.0"), Some((10, 0)));
        assert_eq!(tmux_version("tmux master"), None);
        assert_eq!(tmux_version(""), None);
    }

    #[test]
    fn recognizes_links_into_an_f0_repo() {
        let tmp = std::env::temp_dir().join(format!("f0-appearance-{}", std::process::id()));
        let repo = tmp.join("f0");
        std::fs::create_dir_all(repo.join("crates/f0")).unwrap();
        std::fs::create_dir_all(repo.join("config/tmux")).unwrap();
        std::fs::create_dir_all(tmp.join("other/config/tmux")).unwrap();
        let rel = Path::new("config/tmux");

        let bundled = tmp.join("bundled");
        std::os::unix::fs::symlink(repo.join(rel), &bundled).unwrap();
        let relative = tmp.join("relative");
        std::os::unix::fs::symlink("f0/config/tmux", &relative).unwrap();
        let foreign = tmp.join("foreign");
        std::os::unix::fs::symlink(tmp.join("other/config/tmux"), &foreign).unwrap();
        let plain = tmp.join("plain");
        std::fs::create_dir_all(&plain).unwrap();

        assert!(is_bundled(&bundled, rel));
        assert!(is_bundled(&relative, rel));
        assert!(!is_bundled(&foreign, rel));
        assert!(!is_bundled(&plain, rel));
        assert!(!is_bundled(&tmp.join("missing"), rel));
        std::fs::remove_dir_all(&tmp).unwrap();
    }
}
