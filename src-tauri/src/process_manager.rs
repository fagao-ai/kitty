use crate::system_interface::interface;
use std::collections::HashMap;
pub struct ProcessManager {
    childs: HashMap<String, u32>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            childs: HashMap::new(),
        }
    }

    pub fn add_child(&mut self, process_name: &str, child_pid: u32) {
        self.childs.insert(process_name.to_string(), child_pid);
    }

    pub fn kill(&mut self, process_name: &str) -> Result<(), anyhow::Error> {
        let child_pid = self.childs.get(process_name);
        match child_pid {
            Some(child_pid) => {
                let result = interface::kill_by_pid(child_pid.clone());
                if result == -1 {
                    println!("无法发送信号");
                } else {
                    self.childs.remove(process_name);
                    println!("信号已发送");
                }
            }
            None => (),
        }

        Ok(())
    }
}
