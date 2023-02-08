use std::process::Command;
use std::os::unix::process::CommandExt;

fn main() {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let err = Command::new("rustc")
        .args(&args)
        .exec();
    println!("Error: {}", err);
}
