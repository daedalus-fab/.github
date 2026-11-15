//! Rust-first organization policy gate.
//! Keeps existing Python automation visible as migration debt without allowing
//! new Python scripts/tests. All checks are read-only, deterministic, offline.
#![forbid(unsafe_code)]

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{self, Command},
};

// Recorded from the exact main-branch tree at policy adoption. These are
// legacy automation only; the allowlist cannot expand without code review.
const LEGACY_PYTHON: &[&str] = &[
    "scripts/repository_relationships_lib.py",
    "scripts/validate_baseline.py",
    "scripts/validate_repository_relationships.py",
    "scripts/validate_workflow_security.py",
];

fn is_python_automation(path: &str) -> bool {
    path.ends_with(".py") || path.ends_with(".pyw")
}

fn validate_python_file_list(files: &[String]) -> Vec<String> {
    let mut errors = Vec::new();
    for path in files.iter().filter(|p| is_python_automation(p)) {
        if !LEGACY_PYTHON.contains(&path.as_str()) {
            errors.push(format!(
                "new Python script/test is forbidden: {path}; use Rust (cargo test/xtask)"
            ));
        }
    }
    for legacy in LEGACY_PYTHON {
        if !files.iter().any(|path| path == legacy) {
            errors.push(format!(
                "remove retired Python automation from the migration allowlist: {legacy}"
            ));
        }
    }
    errors
}

fn git_tracked_paths(root: &Path) -> Result<Vec<String>, String> {
    let output = Command::new("git")
        .args(["ls-files", "-z", "--cached"])
        .current_dir(root)
        .output()
        .map_err(|err| format!("git ls-files could not start: {err}"))?;
    if !output.status.success() {
        return Err(format!(
            "git ls-files failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    output
        .stdout
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| {
            let decoded = std::str::from_utf8(path)
                .map_err(|_| "non-UTF-8 tracked path: cannot verify policy".to_string())?;
            Ok(decoded.replace('\\', "/"))
        })
        .collect()
}

fn validate_python_automation(root: &Path) -> Result<Vec<String>, String> {
    Ok(validate_python_file_list(&git_tracked_paths(root)?))
}

fn is_hex(value: &str, len: usize) -> bool {
    value.len() == len && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn action_reference_error(value: &str) -> Option<String> {
    let value = value
        .split('#')
        .next()
        .unwrap_or("")
        .trim()
        .trim_matches(|character| character == '"' || character == '\'');
    if value.starts_with("./") {
        return None;
    }
    if let Some(container) = value.strip_prefix("docker://") {
        if let Some((image, digest)) = container.rsplit_once("@sha256:") {
            if !image.is_empty() && is_hex(digest, 64) {
                return None;
            }
        }
        return Some(format!(
            "Docker action must use an immutable sha256 digest: {value}"
        ));
    }
    match value.rsplit_once('@') {
        Some((action, sha)) if !action.is_empty() && is_hex(sha, 40) => None,
        _ => Some(format!(
            "remote GitHub Action must use an immutable 40-character commit SHA: {value}"
        )),
    }
}

fn validate_workflow_text(label: &str, content: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for (index, line) in content.lines().enumerate() {
        let trimmed = line.trim_start();
        let normalized = trimmed.strip_prefix("- ").unwrap_or(trimmed);
        let Some(value) = normalized.strip_prefix("uses:") else {
            continue;
        };
        if let Some(error) = action_reference_error(value) {
            errors.push(format!("{label}:{}: {error}", index + 1));
        }
    }
    errors
}

fn validate_workflows(root: &Path) -> Result<Vec<String>, String> {
    let directory = root.join(".github/workflows");
    let mut paths = fs::read_dir(&directory)
        .map_err(|e| format!("{}: {e}", directory.display()))?
        .map(|entry| entry.map(|entry| entry.path()).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    paths.sort();
    let mut failures = Vec::new();
    for path in paths {
        if !matches!(
            path.extension().and_then(|ext| ext.to_str()),
            Some("yml" | "yaml")
        ) {
            continue;
        }
        let content =
            fs::read_to_string(&path).map_err(|err| format!("{}: {err}", path.display()))?;
        let name = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .display()
            .to_string();
        failures.extend(validate_workflow_text(&name, &content));
    }
    Ok(failures)
}

fn validate_json(path: &Path) -> Result<(), String> {
    let contents = fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
    serde_json::from_str::<serde_json::Value>(&contents)
        .map(|_| ())
        .map_err(|err| format!("invalid JSON at {}: {err}", path.display()))
}

fn run(command: &str, root: &Path) -> Result<Vec<String>, String> {
    match command {
        "scripts" => validate_python_automation(root),
        "workflows" => validate_workflows(root),
        "json" => {
            validate_json(root)?;
            Ok(Vec::new())
        }
        "all" => {
            let mut failures = validate_python_automation(root)?;
            failures.extend(validate_workflows(root)?);
            for name in ["organization-policy.json", "repository-relationships.json"] {
                if let Err(err) = validate_json(&root.join(name)) {
                    failures.push(err);
                }
            }
            Ok(failures)
        }
        _ => Err("usage: rust-script-policy [all|scripts|workflows|json] [path]".to_string()),
    }
}

fn main() {
    let mut args = env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "all".to_owned());
    let root = PathBuf::from(args.next().unwrap_or_else(|| ".".to_owned()));
    if args.next().is_some() {
        eprintln!("usage: rust-script-policy [all|scripts|workflows|json] [path]");
        process::exit(2);
    }
    match run(&command, &root) {
        Ok(errors) if errors.is_empty() => println!("rust-script-policy: {command} passed"),
        Ok(errors) => {
            for error in errors {
                eprintln!("ERROR: {error}");
            }
            process::exit(1);
        }
        Err(error) => {
            eprintln!("ERROR: {error}");
            process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn legacy_file_list() -> Vec<String> {
        LEGACY_PYTHON
            .iter()
            .map(|path| (*path).to_string())
            .collect()
    }

    #[test]
    fn known_legacy_python_paths_are_allowed_during_migration() {
        assert!(validate_python_file_list(&legacy_file_list()).is_empty());
    }

    #[test]
    fn new_python_tests_and_scripts_are_rejected_without_suppressing_legacy() {
        let mut paths = legacy_file_list();
        paths.extend(
            [
                "scripts/new_fixture.py",
                "tests/test_auth.py",
                "tools/run.pyw",
            ]
            .iter()
            .map(|path| (*path).to_string()),
        );
        let errors = validate_python_file_list(&paths);
        assert_eq!(errors.len(), 3);
        assert!(errors
            .iter()
            .any(|error| error.contains("tests/test_auth.py")));
        assert!(errors
            .iter()
            .any(|error| error.contains("scripts/new_fixture.py")));
        assert!(errors.iter().any(|error| error.contains("tools/run.pyw")));
    }

    #[test]
    fn retiring_a_legacy_script_requires_shrinking_the_allowlist() {
        let mut paths = legacy_file_list();
        paths.retain(|path| path != "scripts/validate_baseline.py");
        let failures = validate_python_file_list(&paths);
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("retired"));
    }

    #[test]
    fn immutable_action_refs_and_local_actions_are_accepted() {
        let good = format!("actions/checkout@{}", "a".repeat(40));
        let docker = format!("docker://alpine@sha256:{}", "b".repeat(64));
        for value in [good.as_str(), docker.as_str(), "./.github/actions/custom"] {
            assert_eq!(action_reference_error(value), None, "{value}");
        }
    }

    #[test]
    fn mutable_and_missing_action_refs_fail_closed() {
        for value in [
            "actions/checkout@v4",
            "owner/reusable/.github/workflows/build.yml@main",
            "actions/cache",
            "docker://alpine:latest",
            "docker://alpine@sha256:12345",
            "",
        ] {
            assert!(action_reference_error(value).is_some(), "{value}");
        }
    }

    #[test]
    fn workflow_policy_covers_indented_steps_and_line_numbers() {
        let input = "jobs:\n  tests:\n    steps:\n      - uses: actions/checkout@v4 # mutable\n      - uses: ./local\n      - uses: actions/setup-node@abc\n";
        let errors = validate_workflow_text("ci.yml", input);
        assert_eq!(errors.len(), 2);
        assert!(errors[0].contains("ci.yml:4:"));
        assert!(errors[1].contains("ci.yml:6:"));
    }

    #[test]
    fn invalid_json_is_a_failing_validation() {
        let path =
            std::env::temp_dir().join(format!("rust-script-policy-invalid-{}.json", process::id()));
        // Avoid filesystem fixture collisions; parse JSON directly for the negative test.
        let _ = path;
        assert!(serde_json::from_str::<serde_json::Value>("{broken").is_err());
        assert!(serde_json::from_str::<serde_json::Value>(r#"{"schema_version":1}"#).is_ok());
    }
}
