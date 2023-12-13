use std::{collections::HashMap, sync::Mutex};
use tauri_plugin_shell::process::CommandChild;

pub struct ProcessManager {
    childs: HashMap<String, Mutex<CommandChild>>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            childs: HashMap::new(),
        }
    }

    pub fn add_child(&mut self, process_name: &str, child: CommandChild) {
        self.childs.insert(process_name.to_string(), Mutex::new(child));
    }

    // fn kill(&mut self, process_name: &str) -> Result<(), std::io::Error> {
    //     // let aa = self.childs.get(process_name);
    //     let child = self.childs.get(process_name).unwrap();
    //     child.lock().unwrap().kill();
    //     Ok(())
    // }

    // fn check(&mut self, process_name: &str) -> Result<(), std::io::Error> {
    //     let child = self.childs.get_mut(process_name).unwrap();
    //     child.inner().
    //     match child.try_wait() {
    //         Some(status) => {
    //             if status.success() {
    //                 println!("Child process exited successfully.");
    //             } else {
    //                 println!("Child process exited with an error: {:?}", status);
    //             }
    //         }
    //         None => {
    //             println!("Child process is still running.");
    //         }
    //     }
    //     Ok(())
    // }
}
