use std::process;

fn check_error(output: &process::Output) {
    match output.status.success() {
        true => (),
        false => {
            println!("{}", String::from_utf8_lossy(&output.stderr).trim());
            process::exit(1)
        }
    }
}

pub fn check_git() {
    println!("status: checking git repository...");
    let output = process::Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .expect("error: failed to check git repository in this directory");

    check_error(&output);
}

pub fn add_all_stage() {
    println!("status: staging all changes in this directory...");
    let output = process::Command::new("git")
        .args(["add", "."])
        .output()
        .expect("error: failed to stage all changes in this directory");

    check_error(&output);
}
