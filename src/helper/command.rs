use std::process;

pub fn run(base: &String, args: &[String]) -> std::process::Output {
    return process::Command::new(&base)
        .args(args)
        .stderr(std::process::Stdio::piped()) 
        .stdout(std::process::Stdio::piped()) 
        .stdin(std::process::Stdio::null()) 
        .output()
        .expect("\x1B[38;5;1m error \x1B[0m failed to run command");
}
