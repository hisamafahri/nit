use std::process;

pub fn handle(output: &process::Output) {
    match output.status.success() {
        true => (), // TODO: print output from successful exit
        false => {
            println!("{}", String::from_utf8_lossy(&output.stderr).trim());
            process::exit(1)
        }
    }
}