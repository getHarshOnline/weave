// SPDX-License-Identifier: AGPL-3.0-or-later

//! Weave's intended graph, checked by the shared repository engine.

mod purity;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::process::Command;

use rtepl_repository::dependencies::{
    check_dependencies, check_external_dependencies, DependencyPolicy, ExternalDependencyPolicy,
};
use rtepl_repository::rust_source::{check_rust_source, RustSourcePolicy};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repository root")
}

fn policy() -> DependencyPolicy {
    let graph: &[(&str, &[&str])] = &[
        ("weave-core", &[]),
        ("weave-authoring", &["weave-core"]),
        ("weave-ports", &["weave-core"]),
        ("weave-application", &["weave-core", "weave-ports"]),
        ("weave-postgres", &["weave-core", "weave-ports"]),
        ("weave-repository-quality", &[]),
    ];
    let production: BTreeMap<String, BTreeSet<String>> = graph
        .iter()
        .map(|(name, dependencies)| {
            (
                (*name).to_owned(),
                dependencies.iter().map(|name| (*name).to_owned()).collect(),
            )
        })
        .collect();
    let mut forbidden = BTreeSet::new();
    // Inner layers cannot acquire outer code even through test dependencies.
    for inner in [
        "weave-core",
        "weave-authoring",
        "weave-ports",
        "weave-application",
    ] {
        for outer in production.keys() {
            if outer != inner && !production[inner].contains(outer) {
                forbidden.insert((inner.to_owned(), outer.clone()));
            }
        }
    }
    DependencyPolicy {
        production,
        forbidden,
        ..DependencyPolicy::default()
    }
}

fn metadata() -> Vec<u8> {
    let output = Command::new(env!("CARGO"))
        .current_dir(root())
        .args([
            "metadata",
            "--locked",
            "--offline",
            "--no-deps",
            "--format-version",
            "1",
        ])
        .output()
        .expect("Cargo metadata");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}

fn external_policy() -> ExternalDependencyPolicy {
    // Every outside production dependency needs an explicit review of its layer.
    policy()
        .production
        .keys()
        .map(|name| (name.clone(), BTreeSet::new()))
        .collect()
}

#[test]
fn dependencies_follow_the_declared_layers() {
    let metadata = metadata();
    let failures = check_dependencies(&root(), &metadata, &policy()).expect("dependency check");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    let failures = check_external_dependencies(&metadata, &external_policy())
        .expect("outside dependency check");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn production_source_keeps_tests_and_unchecked_calls_out() {
    let root = root();
    let rules = RustSourcePolicy {
        test_functions: true,
        inline_test_modules: true,
        forbidden_methods: &["unwrap", "unwrap_err", "expect", "expect_err"],
    };
    let entries = rtepl_repository::source_entries(&root, &[".git", "target", "tests"])
        .expect("source inventory");
    for entry in entries {
        assert!(
            !entry.file_type.is_symlink(),
            "source symlink: {}",
            entry.path.display()
        );
        if entry
            .path
            .extension()
            .is_some_and(|extension| extension == "rs")
        {
            let source = std::fs::read_to_string(&entry.path).expect("Rust source");
            let failures = check_rust_source(&source, &rules).expect("valid Rust syntax");
            assert!(
                failures.is_empty(),
                "{}: {}",
                entry.path.display(),
                failures.join("; ")
            );
            assert!(
                source.lines().count() < 1500,
                "split {} by concern",
                entry.path.display()
            );
        }
    }
}
