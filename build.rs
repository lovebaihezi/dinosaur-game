use std::process::Command;

fn get_version() {
    let version_output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();

    let git_hash = String::from_utf8(version_output.stdout).unwrap();

    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}

fn get_branch() {
    let branch_output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .unwrap();

    let git_branch = String::from_utf8(branch_output.stdout).unwrap();

    println!("cargo:rustc-env=GIT_BRANCH={}", git_branch);
}

fn get_build_date() {
    let build_date = chrono::Local::now().to_rfc2822();

    println!("cargo:rustc-env=BUILD_DATE={}", build_date);
}

fn main() {
    get_version();
    get_branch();
    get_build_date();
}
