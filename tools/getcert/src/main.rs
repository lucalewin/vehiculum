use std::process::{Command, ExitCode, ExitStatus};

fn main() {
    println!("starting rooted adb process");

    let _devices = get_devices();

    // println!("output: {:?}", result);
}

fn get_devices() -> Vec<String> {
    let output = Command::new("adb")
       .arg("devices").output().unwrap();

    if !output.status.success() {
        panic!("[adb devices] -> unexpected exit code: {:#?}", output.status)
    }
    
    let output_as_string = std::str::from_utf8(output.stdout.as_slice()).unwrap();

    let lines: Vec<String> = output_as_string.split_whitespace().split("\r\n")
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();

    lines.iter().map(|line| line.split("\t").fi).collect();

    println!("[adb devices] -> {:#?}", lines);

    Vec::new()
}
