use std::{env, process::Command};

const POWERSHELL_DATE_COMMAND: &str = "(Get-Date).ToString('yyyy-MM-dd')";

fn get_version() {
    let version_output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();

    let git_hash = String::from_utf8(version_output.stdout)
        .unwrap()
        .trim()
        .to_string();

    println!("cargo:rustc-env=GIT_HASH={git_hash}");
}

fn get_branch() {
    let branch_output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .unwrap();

    let git_branch = String::from_utf8(branch_output.stdout)
        .unwrap()
        .trim()
        .to_string();

    println!("cargo:rustc-env=GIT_BRANCH={git_branch}");
}

fn get_build_date() {
    if cfg!(target_os = "windows") {
        let build_date = Command::new("pwsh")
            .args(["-Command", "(Get-Date).ToString('yyyy-MM-dd HH:mm:ss')"])
            .output()
            .unwrap();

        let date = String::from_utf8(build_date.stdout).unwrap();

        println!("cargo:rustc-env=BUILD_DATE={date}");
    } else {
        let build_date = Command::new("date")
            .args(["+%Y-%m-%d %H:%M:%S"])
            .output()
            .unwrap();

        let date = String::from_utf8(build_date.stdout).unwrap();

        println!("cargo:rustc-env=BUILD_DATE={date}");
    }
}

fn get_date() -> Option<String> {
    if cfg!(target_os = "windows") {
        let build_date = Command::new("pwsh")
            .args(["-Command", POWERSHELL_DATE_COMMAND])
            .output()
            .ok()
            .or_else(|| {
                Command::new("powershell")
                    .args(["-Command", POWERSHELL_DATE_COMMAND])
                    .output()
                    .ok()
            })?;

        Some(
            String::from_utf8(build_date.stdout)
                .ok()?
                .trim()
                .to_string(),
        )
    } else {
        let build_date = Command::new("date").args(["+%Y-%m-%d"]).output().ok()?;

        Some(
            String::from_utf8(build_date.stdout)
                .ok()?
                .trim()
                .to_string(),
        )
    }
}

fn get_short_git_hash() -> Option<String> {
    let version_output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()?;

    Some(
        String::from_utf8(version_output.stdout)
            .ok()?
            .trim()
            .to_string(),
    )
}

fn get_game_version() {
    let github_event = env::var("GITHUB_EVENT_NAME").unwrap_or_default();
    let github_ref = env::var("GITHUB_REF").unwrap_or_default();
    let cargo_version = env::var("CARGO_PKG_VERSION").unwrap_or_default();
    let fallback = || get_short_git_hash().unwrap_or_else(|| cargo_version.clone());

    let version = if github_event == "schedule" {
        get_date().unwrap_or_else(|| cargo_version.clone())
    } else if github_ref.starts_with("refs/tags/") {
        cargo_version.clone()
    } else if github_ref == "refs/heads/main" {
        fallback()
    } else if let Some(pr_part) = github_ref.strip_prefix("refs/pull/") {
        let pr_number = pr_part.split('/').next().unwrap_or_default();
        if pr_number.is_empty() {
            fallback()
        } else {
            pr_number
                .parse::<u64>()
                .filter(|value| *value > 0)
                .map(|value| format!("pr-{value}"))
                .unwrap_or_else(|| fallback())
        }
    } else {
        fallback()
    };

    println!("cargo:rustc-env=GAME_VERSION={version}");
}

fn main() {
    get_version();
    get_branch();
    get_build_date();
    get_game_version();
}
