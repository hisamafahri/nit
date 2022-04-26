use std::process;

pub fn print(output: &process::Output) {
    match output.status.success() {
        true => println!("{}", String::from_utf8_lossy(&output.stdout)),
        false => {
            println!("\x1B[38;5;1m error \x1B[0m {}", String::from_utf8_lossy(&output.stdout));
            process::exit(1)
        }
    }
}

pub fn handle(output: &process::Output) -> String {{
    match output.status.success() {
        true => format!("{}", String::from_utf8_lossy(&output.stdout)),
        false => {
            println!("\x1B[38;5;1m error \x1B[0m {}", String::from_utf8_lossy(&output.stdout));
            process::exit(1)
        }
    }
}}

pub fn ignore(output: &process::Output) {{
    match output.status.success() {
        true => (),
        false => {
            println!("\x1B[38;5;1m error \x1B[0m {}", String::from_utf8_lossy(&output.stdout));
            process::exit(1)
        }
    }
}}
