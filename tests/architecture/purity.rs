// SPDX-License-Identifier: AGPL-3.0-or-later

//! Weave's pure layers and protected result types, exercised against forbidden changes.

use std::path::Path;

use rtepl_repository::dependencies::{
    check_dependencies, check_external_dependencies_in_scope, DependencyScope,
    ExternalDependencyPolicy,
};
use rtepl_repository::purity::{check_purity_files, ForbiddenImplementation, PurityPolicy};
use serde_json::{json, Value};

const PURE_CRATES: &[&str] = &["weave-core", "weave-authoring"];

const EFFECT_PATHS: &[&str] = &[
    "std::fs",
    "std::net",
    "std::env",
    "std::io",
    "std::process",
    "std::thread",
    "std::time::SystemTime::now",
    "std::time::Instant::now",
    "std::random",
    "rand",
    "rand_core",
    "getrandom",
    "fastrand",
    "tokio",
    "async_std",
    "smol",
];

// These are planned types, not claims that the crates implement them.
const IMPLEMENTATIONS: &[ForbiddenImplementation<'_>] = &[
    ForbiddenImplementation {
        type_name: "Refusal",
        trait_name: "From",
    },
    ForbiddenImplementation {
        type_name: "CommandOutcome",
        trait_name: "From",
    },
    ForbiddenImplementation {
        type_name: "CommitReceipt",
        trait_name: "Deserialize",
    },
    ForbiddenImplementation {
        type_name: "CommitReceipt",
        trait_name: "Default",
    },
    ForbiddenImplementation {
        type_name: "CommitReceipt",
        trait_name: "From",
    },
];

fn rules<'a>(name: &str, exports: &'a [&'a str]) -> PurityPolicy<'a> {
    PurityPolicy {
        forbidden_paths: if PURE_CRATES.contains(&name) {
            EFFECT_PATHS
        } else {
            &[]
        },
        forbidden_reexports: if name == "weave-application" {
            exports
        } else {
            &[]
        },
        forbidden_implementations: IMPLEMENTATIONS,
    }
}

fn metadata() -> Value {
    serde_json::from_slice(&super::metadata()).expect("Cargo metadata JSON")
}

fn external_pure_policy() -> ExternalDependencyPolicy {
    super::external_policy()
        .into_iter()
        .filter(|(name, _)| PURE_CRATES.contains(&name.as_str()))
        .collect()
}

fn adapter_exports(metadata: &Value) -> Vec<String> {
    let root = super::root();
    let mut exports = vec!["weave_ports".to_owned()];
    for package in metadata["packages"].as_array().expect("packages") {
        let manifest = Path::new(package["manifest_path"].as_str().expect("manifest"));
        if manifest.starts_with(root.join("adapters")) {
            exports.push(
                package["name"]
                    .as_str()
                    .expect("package name")
                    .replace('-', "_"),
            );
        }
    }
    exports
}

#[test]
fn production_crates_follow_their_purity_rules() {
    let metadata = metadata();
    let failures = check_external_dependencies_in_scope(
        &serde_json::to_vec(&metadata).expect("metadata JSON"),
        &external_pure_policy(),
        DependencyScope::AllDeclarations,
    )
    .expect("pure layers include dev declarations");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    let exports = adapter_exports(&metadata);
    let exports: Vec<&str> = exports.iter().map(String::as_str).collect();
    let root = super::root();
    for package in metadata["packages"].as_array().expect("packages") {
        let name = package["name"].as_str().expect("name");
        let directory = Path::new(package["manifest_path"].as_str().expect("manifest"))
            .parent()
            .expect("crate directory");
        if directory.starts_with(root.join("domains")) {
            assert!(PURE_CRATES.contains(&name), "declare pure domain {name}");
        }
        let mut sources = Vec::new();
        for entry in
            rtepl_repository::source_entries(&directory.join("src"), &[]).expect("crate sources")
        {
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
                sources.push((
                    entry.path.display().to_string(),
                    std::fs::read_to_string(entry.path).expect("source"),
                ));
            }
        }
        let build = directory.join("build.rs");
        if build.exists() {
            sources.push((
                build.display().to_string(),
                std::fs::read_to_string(build).expect("build source"),
            ));
        }
        let sources: Vec<(&str, &str)> = sources
            .iter()
            .map(|(path, text)| (path.as_str(), text.as_str()))
            .collect();
        let failures = check_purity_files(&sources, &rules(name, &exports)).expect("valid Rust");
        assert!(failures.is_empty(), "{name}: {}", failures.join("\n"));
    }
}

#[test]
fn pure_layers_reject_effects_even_through_aliases() {
    let cases = [
        "fn read() { std::fs::read(\"x\"); }",
        "use std::net as network; fn connect() { network::TcpStream::connect(\"x\"); }",
        "fn read() { std::env::var(\"x\"); }",
        "fn read() { std::io::stdin(); }",
        "fn run() { std::process::Command::new(\"x\"); }",
        "use std::time::SystemTime as Clock; fn now() { Clock::now(); }",
        "type Clock = std::time::Instant; fn now() { Clock::now(); }",
        "fn random() { rand::random::<u64>(); }",
        "fn random() { getrandom::fill(&mut []); }",
        "use tokio as runtime; fn run() { runtime::spawn(async {}); }",
    ];
    for name in PURE_CRATES {
        let policy = rules(name, &[]);
        for source in cases {
            let failures =
                check_purity_files(&[("effect.rs", source)], &policy).expect("valid fixture");
            assert!(
                failures
                    .iter()
                    .any(|failure| failure.contains("forbidden path")),
                "{name} accepted {source}"
            );
        }
        let failures = check_purity_files(
            &[
                ("clock.rs", "pub type Clock = std::time::SystemTime;"),
                ("decision.rs", "fn decide() { crate::clock::Clock::now(); }"),
            ],
            &policy,
        )
        .expect("valid cross-file fixture");
        assert!(
            !failures.is_empty(),
            "{name} accepted a cross-file ambient clock"
        );
        let control = "fn decide(read_set: &[u64], instant: u64) -> u64 { read_set.iter().sum::<u64>() + instant }";
        assert!(check_purity_files(&[("decision.rs", control)], &policy)
            .expect("pure fixture")
            .is_empty());
    }
}

#[test]
fn application_cannot_export_ports_or_adapters() {
    let exports = adapter_exports(&metadata());
    let exports: Vec<&str> = exports.iter().map(String::as_str).collect();
    for name in ["weave-application"] {
        let policy = rules(name, &exports);
        for dependency in &exports {
            for source in [
                format!("pub use {dependency}::Handle;"),
                format!("use {dependency}::Handle as Hidden; pub type Public = Hidden;"),
                format!("pub fn handle() -> {dependency}::Handle {{ loop {{}} }}"),
            ] {
                let failures =
                    check_purity_files(&[("export.rs", &source)], &policy).expect("valid fixture");
                assert!(
                    failures
                        .iter()
                        .any(|failure| failure.contains("forbidden public export")),
                    "{name} accepted {source}"
                );
            }
        }
        let control = "use weave_ports::Handle; fn internal(_: Handle) {} pub use weave_authoring as authoring;";
        assert!(check_purity_files(&[("facade.rs", control)], &policy)
            .expect("private port fixture")
            .is_empty());
    }
}

#[test]
fn outcomes_and_receipts_reject_implicit_construction() {
    let policy = rules("weave-application", &[]);
    for rule in IMPLEMENTATIONS {
        let source = if rule.trait_name == "From" {
            format!(
                "impl From<()> for {} {{ fn from(_: ()) -> Self {{ loop {{}} }} }}",
                rule.type_name
            )
        } else {
            format!(
                "#[derive({})] struct {} {{ private: () }}",
                rule.trait_name, rule.type_name
            )
        };
        let failures =
            check_purity_files(&[("receipt.rs", &source)], &policy).expect("valid fixture");
        assert!(
            failures
                .iter()
                .any(|failure| failure.contains("forbidden") && failure.contains(rule.type_name)),
            "accepted {source}"
        );
    }
    let control = "pub struct CommitReceipt { private: () } pub enum CommandOutcome { Rejected, Conflict, Committed(CommitReceipt), CommitUnknown }";
    assert!(check_purity_files(&[("outcomes.rs", control)], &policy)
        .expect("explicit types fixture")
        .is_empty());
}

#[test]
fn pure_layers_reject_new_external_dependencies() {
    let original = metadata();
    for name in PURE_CRATES {
        for kind in [Value::Null, json!("build"), json!("dev")] {
            let mut changed = original.clone();
            let package = changed["packages"]
                .as_array_mut()
                .expect("packages")
                .iter_mut()
                .find(|package| package["name"] == *name)
                .expect("pure package");
            package["dependencies"]
                .as_array_mut()
                .expect("dependencies")
                .push(json!({
                    "name": "tokio", "path": null, "kind": kind,
                    "optional": true, "target": "cfg(unix)", "rename": "runtime"
                }));
            let failures = check_external_dependencies_in_scope(
                &serde_json::to_vec(&changed).expect("fixture JSON"),
                &external_pure_policy(),
                DependencyScope::AllDeclarations,
            )
            .expect("external policy");
            assert!(
                failures
                    .iter()
                    .any(|failure| failure.contains(name) && failure.contains("tokio")),
                "{name} accepted an outside runtime"
            );
        }
    }
}

#[test]
fn domain_and_storage_cannot_gain_runtime_edges() {
    let original = metadata();
    for (caller, provider) in [
        ("weave-authoring", "weave-ports"),
        ("weave-authoring", "weave-application"),
        ("weave-postgres", "weave-application"),
    ] {
        let mut changed = original.clone();
        let packages = changed["packages"].as_array_mut().expect("packages");
        let manifest = packages
            .iter()
            .find(|package| package["name"] == provider)
            .expect("provider")["manifest_path"]
            .as_str()
            .expect("manifest")
            .to_owned();
        let path = Path::new(&manifest).parent().expect("provider directory");
        let package = packages
            .iter_mut()
            .find(|package| package["name"] == caller)
            .expect("caller");
        package["dependencies"]
            .as_array_mut()
            .expect("dependencies")
            .push(json!({"name": provider, "path": path, "kind": null}));
        let failures = check_dependencies(
            &super::root(),
            &serde_json::to_vec(&changed).expect("fixture JSON"),
            &super::policy(),
        )
        .expect("graph policy");
        assert!(!failures.is_empty(), "accepted {caller} -> {provider}");
    }
}
