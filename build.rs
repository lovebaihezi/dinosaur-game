use std::process::Command;

fn main() {
    let version_output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(version_output.stdout).unwrap();

    let branch_output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .unwrap();

    let git_branch = String::from_utf8(branch_output.stdout).unwrap();

    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    println!("cargo:rustc-env=GIT_BRANCH={}", git_branch);
}
