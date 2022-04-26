use std::process;

pub fn run(base: &String, args: &[String]) -> std::process::Output {
    return process::Command::new(&base)
        .args(args)
        .stderr(std::process::Stdio::null()) // don't care about stderr
        .stdout(std::process::Stdio::piped()) // set up stdout so we can read it
        .stdin(std::process::Stdio::piped()) // set up stdin so we can write on it
        .output()
        .expect("error: failed to run command");
}
