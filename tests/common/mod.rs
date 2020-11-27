use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};

#[allow(unused)]
pub fn spawn_child(command: &str, args: &[&str]) {
    let mut cmd_object = Command::new("cargo");
    let mut cmd_ref = cmd_object
        .arg("run")
        .arg("--bin")
        .arg("test")
        .arg("--")
        .arg(command);
    for arg in args {
        cmd_ref = cmd_ref.arg(arg);
    }
    let child = cmd_ref.output().expect("failed to execute child");

    println!("Child output:");
    std::io::stdout().write_all(&child.stdout).unwrap();
    std::io::stdout().write_all(&child.stderr).unwrap();
    assert_eq!(child.status.code().expect("No return value"), 0);
}

#[allow(unused)]
pub fn start_child_and_wait_for_threads(num: usize) -> Child {
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("test")
        .arg("--")
        .arg("spawn_and_wait")
        .arg(format!("{}", num))
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    {
        let mut f = BufReader::new(child.stdout.as_mut().expect("Can't open stdout"));
        let mut lines = 0;
        while lines < num {
            let mut buf = String::new();
            match f.read_line(&mut buf) {
                Ok(_) => {
                    if buf == "1\n" {
                        lines += 1;
                    }
                }
                Err(e) => {
                    panic!(e);
                }
            }
        }
    }
    child
}