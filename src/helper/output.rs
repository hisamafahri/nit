use std::process;

pub fn print(output: &process::Output) {
    match output.status.success() {
        true => println!("{}", String::from_utf8_lossy(&output.stdout)),
        false => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            process::exit(1)
        }
    }
}

pub fn handle(output: &process::Output) -> String {{
    match output.status.success() {
        true => format!("{}", String::from_utf8_lossy(&output.stdout)),
        false => {
            format!("{}", String::from_utf8_lossy(&output.stdout));
            process::exit(1)
        }
    }
}}

pub fn ignore(output: &process::Output) {{
    match output.status.success() {
        true => (),
        false => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            process::exit(1)
        }
    }
}}
