use std::process;

pub fn handler(output: &process::Output, on_success: ()) {
    match output.status.success() {
        true => on_success,
        false => {
            println!("{}", String::from_utf8_lossy(&output.stderr).trim());
            process::exit(1)
        }
    }
}

pub fn handler_string(output: &process::Output) -> String {{
    match output.status.success() {
        true => format!("{}", String::from_utf8_lossy(&output.stdout).trim()),
        false => {
            println!("{}", String::from_utf8_lossy(&output.stderr).trim());
            process::exit(1)
        }
    }
}}
