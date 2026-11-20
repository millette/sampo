//! Publishing through a stand-in `nub`.
//!
//! Its own test binary: it overrides `PATH` process-wide.
#![cfg(unix)]

mod common;

use sampo_core::adapters::{PackageAdapter, PublishOutcome};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

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

/// nub refuses a dirty checkout or a non-release branch unless told not to check;
/// `git.release_branches` already vets the branch.
#[test]
fn a_nub_package_publishes_with_nub_and_git_checks_off() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    let argv_log = root.join("nub-argv.log");
    let bin = root.join("fake-bin");

    let manifest = root.join("package.json");
    fs::write(
        &manifest,
        r#"{"name":"pkg","version":"1.0.0","devEngines":{"packageManager":{"name":"nub","onFail":"error"}}}"#,
    )
    .unwrap();
    write_fake_nub(&bin, &argv_log);
    let _guard = common::use_only(&bin);

    let outcome = PackageAdapter::Npm
        .publish(&manifest, false, &[])
        .expect("the stand-in nub succeeds");

    assert_eq!(outcome, PublishOutcome::Ran);
    assert_eq!(
        fs::read_to_string(&argv_log).unwrap().trim(),
        "publish --no-git-checks"
    );
}

/// nub trades the CI identity for a publish token itself, so it needs no detour through npm.
#[test]
fn trusted_publishing_keeps_nub_publish() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path();
    let argv_log = root.join("nub-argv.log");
    let bin = root.join("fake-bin");

    let manifest = root.join("package.json");
    fs::write(
        &manifest,
        r#"{"name":"pkg","version":"1.0.0","packageManager":"nub@0.11.1"}"#,
    )
    .unwrap();
    write_fake_nub(&bin, &argv_log);
    let _guard = common::use_only(&bin);
    unsafe {
        std::env::set_var("NPM_ID_TOKEN", "id-token");
        for token_var in [
            "NPM_TOKEN",
            "NODE_AUTH_TOKEN",
            "NPM_CONFIG_TOKEN",
            "npm_config_token",
        ] {
            std::env::remove_var(token_var);
        }
    }

    PackageAdapter::Npm
        .publish(&manifest, false, &[])
        .expect("the stand-in nub succeeds");

    assert_eq!(
        fs::read_to_string(&argv_log).unwrap().trim(),
        "publish --no-git-checks"
    );
}
