use std::process;
use futures_lite::future;

pub fn pull(remote: &String, branch: &String) {
    println!("\x1B[38;5;245m info \x1B[0m pulling changes...");
    future::block_on(async {
        let cmd = process::Command::new("git")
            .args(["pull", &remote.trim(), &branch.trim()])
            .stderr(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stdin(std::process::Stdio::null())
            .output()
            .expect("\x1B[38;5;1m error \x1B[0m failed to run command");

        println!("{}", String::from_utf8_lossy(&cmd.stderr).trim());
        println!("{}", String::from_utf8_lossy(&cmd.stdout).trim());
    });
}
