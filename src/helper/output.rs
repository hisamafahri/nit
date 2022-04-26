use std::process;

pub fn print(output: &process::Output) {
    match output.status.success() {
        true => println!("{}", String::from_utf8_lossy(&output.stdout).trim()),
        false => {
            println!("{}", String::from_utf8_lossy(&output.stderr).trim());
            process::exit(1)
        }
    }
}

pub fn print_out(output: &process::Output) {
    match output.status.success() {
        true => println!("{}", String::from_utf8_lossy(&output.stdout).trim()),
        false => {
            println!("{}", String::from_utf8_lossy(&output.stdout).trim());
            process::exit(1)
        }
    }
}

pub fn handle(output: &process::Output) -> String {{
    match output.status.success() {
        true => format!("{}", String::from_utf8_lossy(&output.stdout).trim()),
        false => {
            println!("{}", String::from_utf8_lossy(&output.stderr).trim());
            process::exit(1)
        }
    }
}}

pub fn ignore(output: &process::Output) {{
    match output.status.success() {
        true => (),
        false => {
            println!("{}", String::from_utf8_lossy(&output.stderr).trim());
            process::exit(1)
        }
    }
}}
