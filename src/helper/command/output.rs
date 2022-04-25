use std::process;

pub fn output_print(output: &process::Output) {
    match output.status.success() {
        true => println!("{}", String::from_utf8_lossy(&output.stdout).trim()),
        false => {
            println!("{}", String::from_utf8_lossy(&output.stderr).trim());
            process::exit(1)
        }
    }
}

pub fn output_handle(output: &process::Output) -> String {{
    match output.status.success() {
        true => format!("{}", String::from_utf8_lossy(&output.stdout).trim()),
        false => {
            println!("{}", String::from_utf8_lossy(&output.stderr).trim());
            process::exit(1)
        }
    }
}}

pub fn output_ignore(output: &process::Output) {{
    match output.status.success() {
        true => (),
        false => {
            println!("{}", String::from_utf8_lossy(&output.stderr).trim());
            process::exit(1)
        }
    }
}}
