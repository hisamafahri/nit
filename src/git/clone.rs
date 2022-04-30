use std::process;
use futures_lite::future;

pub fn clone(url: &String) {
    println!("\x1B[38;5;245m info \x1B[0m cloning repository...");
    future::block_on(async {
        let cmd = process::Command::new("git")
            .args(["clone", &url.trim()])
            .stderr(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stdin(std::process::Stdio::null())
            .output()
            .expect("\x1B[38;5;1m error \x1B[0m failed to run command");

            if cmd.status.success() {
                println!("{}", String::from_utf8_lossy(&cmd.stderr).trim());
                println!("\x1B[38;5;2m success \x1B[0m repository cloned successfully!");
                process::exit(1)
            }
            println!("\x1B[38;5;1m error \x1B[0m {}", String::from_utf8_lossy(&cmd.stderr).trim());
    });
}
