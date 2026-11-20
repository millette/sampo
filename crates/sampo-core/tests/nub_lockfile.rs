//! The release path against a stand-in `nub`.
//!
//! Its own test binary: it overrides `PATH` process-wide.
#![cfg(unix)]

mod common;

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use sampo_core::run_release;

/// The bun workspace fixture, switched to nub by its lockfile.
fn write_nub_workspace(root: &Path) {
    common::write_workspace(root);
    fs::remove_file(root.join("bun.lock")).unwrap();
    fs::write(root.join("nub.lock"), "lockfileVersion: '9.0'\n").unwrap();
}

fn write_fake_nub(dir: &Path, argv_log: &Path) {
    fs::create_dir_all(dir).unwrap();
    let stub = dir.join("nub");
    fs::write(
        &stub,
        format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" >> '{}'\nexit 0\n",
            argv_log.display()
        ),
    )
    .unwrap();
    fs::set_permissions(&stub, fs::Permissions::from_mode(0o755)).unwrap();
}

#[test]
fn release_refreshes_nub_lock_with_nub() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    let argv_log = root.join("nub-argv.log");
    let bin = root.join("fake-bin");

    write_nub_workspace(root);
    write_fake_nub(&bin, &argv_log);
    let _guard = common::use_only(&bin);

    run_release(root, false).expect("the release must succeed with a working nub");

    assert_eq!(
        fs::read_to_string(&argv_log).unwrap().trim(),
        "install --lockfile-only --fix-lockfile",
        "the exact argv handed to nub"
    );
    assert!(
        fs::read_to_string(root.join("packages/pkg-a/package.json"))
            .unwrap()
            .contains("\"1.1.0\""),
        "the release must bump the member"
    );
}
