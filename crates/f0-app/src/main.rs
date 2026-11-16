//! The executable of f0.app, a copy of Ghostty.app that install.sh renames. macOS runs it
//! when the app opens; it replaces itself with the Ghostty next to it, told to read only
//! f0's Ghostty config. Finder passes no arguments, and Ghostty has no other way to pick
//! a config file, so the flags have to come from here. A shell script cannot take this
//! place: launchd refuses to start an app whose executable is a script (POSIX 162).
//!
//! `XDG_CONFIG_HOME` is left alone on purpose: the panes inherit Ghostty's environment,
//! and f0 reads its own config from there.

use std::ffi::OsString;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let Some(ghostty) = ghostty_path() else {
        eprintln!("f0.app: cannot find the Ghostty executable next to this one");
        return ExitCode::FAILURE;
    };
    let Some(config) = config_file(
        std::env::var_os("F0_CONFIG_DIR"),
        std::env::var_os("XDG_CONFIG_HOME"),
        std::env::var_os("HOME"),
    ) else {
        eprintln!("f0.app: HOME is not set, so f0's Ghostty config cannot be found");
        return ExitCode::FAILURE;
    };
    let error = Command::new(&ghostty)
        .args(ghostty_args(&config, std::env::args_os().skip(1)))
        .exec();
    eprintln!("f0.app: could not start {}: {error}", ghostty.display());
    ExitCode::FAILURE
}

/// `Contents/MacOS/ghostty`, beside this executable.
fn ghostty_path() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?.canonicalize().ok()?;
    Some(exe.parent()?.join("ghostty"))
}

/// `ghostty.conf` in f0's config folder, found the way the f0 CLI finds it:
/// `F0_CONFIG_DIR`, else `$XDG_CONFIG_HOME/f0`, else `~/.config/f0`. Empty variables
/// count as unset.
fn config_file(
    f0_dir: Option<OsString>,
    xdg: Option<OsString>,
    home: Option<OsString>,
) -> Option<PathBuf> {
    let set = |value: Option<OsString>| value.filter(|v| !v.is_empty()).map(PathBuf::from);
    let dir = match (set(f0_dir), set(xdg)) {
        (Some(dir), _) => dir,
        (None, Some(xdg)) => xdg.join("f0"),
        (None, None) => set(home)?.join(".config/f0"),
    };
    Some(dir.join("ghostty.conf"))
}

/// Only f0's config, then whatever the app was opened with. macOS may add a `-psn_`
/// process serial number, which Ghostty would reject.
fn ghostty_args(config: &Path, args: impl Iterator<Item = OsString>) -> Vec<OsString> {
    let mut flag = OsString::from("--config-file=");
    flag.push(config);
    let mut out = vec![OsString::from("--config-default-files=false"), flag];
    out.extend(args.filter(|arg| !arg.to_string_lossy().starts_with("-psn_")));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn os(value: &str) -> Option<OsString> {
        Some(OsString::from(value))
    }

    #[test]
    fn config_follows_the_f0_cli() {
        assert_eq!(
            config_file(None, None, os("/Users/me")),
            Some(PathBuf::from("/Users/me/.config/f0/ghostty.conf"))
        );
        assert_eq!(
            config_file(None, os("/x"), os("/Users/me")),
            Some(PathBuf::from("/x/f0/ghostty.conf"))
        );
        assert_eq!(
            config_file(os("/f0"), os("/x"), os("/Users/me")),
            Some(PathBuf::from("/f0/ghostty.conf"))
        );
        assert_eq!(
            config_file(os(""), os(""), os("/Users/me")),
            Some(PathBuf::from("/Users/me/.config/f0/ghostty.conf"))
        );
        assert_eq!(config_file(None, None, None), None);
    }

    #[test]
    fn args_load_only_the_f0_config_and_drop_the_serial_number() {
        let args = ["-psn_0_12345", "--title=x"]
            .map(OsString::from)
            .into_iter();
        assert_eq!(
            ghostty_args(Path::new("/c/ghostty.conf"), args),
            [
                "--config-default-files=false",
                "--config-file=/c/ghostty.conf",
                "--title=x"
            ]
            .map(OsString::from)
        );
    }
}
