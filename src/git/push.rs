use std::process;
use futures_lite::future;

pub fn push(remote: &String, branch: &String) {
    println!("\x1B[7m STATUS: \x1B[0m pushing changes...");
    future::block_on(async {
        let cmd = process::Command::new("git")
            .args(["push", &remote.trim(), &branch.trim()])
            .stderr(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stdin(std::process::Stdio::null())
            .output()
            .expect("\x1B[41;1m\x1B[37;1m ERROR: \x1B[0m\x1B[0m failed to run command");

        println!("{}", String::from_utf8_lossy(&cmd.stderr).trim());
        println!("{}", String::from_utf8_lossy(&cmd.stdout).trim());
    });
}
