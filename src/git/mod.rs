use std::process::Command;

pub fn check_git() -> bool {
    println!("status: checking git repository...");
    let output = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .expect("error: failed to check git repository in this directory");

    match output.status.success() {
        true => true,
        false => {
            println!("{}", String::from_utf8_lossy(&output.stderr).trim());
            false
        }
    }
}
