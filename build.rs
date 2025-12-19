use std::process::Command;

fn main() {
    let commit = get_git_commit();
    let build_date = get_build_date();

    println!("cargo:rustc-env=TABSSH_BUILD_COMMIT={}", commit);
    println!("cargo:rustc-env=TABSSH_BUILD_DATE={}", build_date);

    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads/");
}

fn get_git_commit() -> String {
    if let Some(commit) = std::env::var("TABSSH_BUILD_COMMIT").ok() {
        if !commit.is_empty() {
            return commit;
        }
    }

    let output = Command::new("git")
        .args(["rev-parse", "--short=8", "HEAD"])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            String::from_utf8_lossy(&out.stdout).trim().to_string()
        }
        _ => "unknown".to_string(),
    }
}

fn get_build_date() -> String {
    if let Some(date) = std::env::var("TABSSH_BUILD_DATE").ok() {
        if !date.is_empty() {
            return date;
        }
    }

    let output = Command::new("date")
        .args(["+%m/%d/%Y at %H:%M:%S"])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            String::from_utf8_lossy(&out.stdout).trim().to_string()
        }
        _ => "unknown".to_string(),
    }
}
