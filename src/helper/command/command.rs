use std::process;

pub fn command_run(base: &String, args: &[String]) -> std::process::Output {
    return process::Command::new(&base)
        .args(args)
        .output()
        .expect("error: failed to run command");
}
