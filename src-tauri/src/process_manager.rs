use std::process::{Command, Child};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

struct ProcessManager {
    child: Option<Child>,
}

impl ProcessManager {
    fn new() -> Self {
        ProcessManager {
            child: None,
        }
    }

    fn spawn(&mut self, command: &str, args: &[&str]) -> Result<(), io::Error> {
        let child = Command::new(command).args(args).spawn()?;
        self.child = Some(child);
        Ok(())
    }

    fn kill(&mut self) -> Result<(), io::Error> {
        if let Some(child) = self.child.as_mut() {
            child.kill()?;
            self.child = None;
        }
        Ok(())
    }

    fn check(&mut self) -> Result<(), io::Error> {
        if let Some(child) = self.child.as_mut() {
            match child.try_wait()? {
                Some(status) => {
                    if status.success() {
                        println!("Child process exited successfully.");
                    } else {
                        println!("Child process exited with an error: {:?}", status);
                    }
                }
                None => {
                    println!("Child process is still running.");
                }
            }
        } else {
            println!("No child process is running.");
        }
        Ok(())
    }
}

fn main() {
    let mut process_manager = ProcessManager::new();

    // 创建子进程
    if let Err(err) = process_manager.spawn("ls", &["-l"]) {
        eprintln!("Failed to spawn child process: {}", err);
    }

    // 等待一段时间
    thread::sleep(Duration::from_secs(5));

    // 关闭子进程
    if let Err(err) = process_manager.kill() {
        eprintln!("Failed to kill child process: {}", err);
    }

    // 检查子进程
    if let Err(err) = process_manager.check() {
        eprintln!("Failed to check child process: {}", err);
    }
}