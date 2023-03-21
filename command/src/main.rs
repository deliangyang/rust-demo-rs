use std::{process::{Command}};

fn main() {
    println!("Hello, world!");

    let status = Command::new("ls")
        .current_dir("/tmp")
        .status()
        .expect("test");
    assert!(status.success());
    println!("{:?}", status);

    let output = Command::new("ls")
        .current_dir("/tmp")
        .output()
        .expect("ok");

    match output.status.code() {
        Some(code) => {
            if code == 0 {
                let lines = String::from_utf8(output.stdout).expect("success");
                for i in lines.split("\n") {
                    println!("{}", i)
                }
            } else {
                let lines = String::from_utf8(output.stderr).expect("error");
                for i in lines.split("\n") {
                    println!("{}", i)
                }
            }
        },
        None => {
            println!("error {:?}", output.status);
        }
    }
}
