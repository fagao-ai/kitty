pub fn kill_by_pid(child_pid: u32) -> i32 {
    #[cfg(not(target_os = "windows"))]
    {
        use libc::{c_int, kill};
        let result = unsafe { kill(child_pid.clone() as i32, libc::SIGTERM as c_int) };
        if result == -1 {
            println!("无法发送信号");
        } else {
            println!("信号已发送");
        }
        return result;
    }

    #[cfg(target_os = "windows")]
    {
        use kernel32::GetLastError;
        use winapi::shared::minwindef::{DWORD, FALSE};
        use winapi::um::handleapi::CloseHandle;
        use winapi::um::processthreadsapi::OpenProcess;
        use winapi::um::processthreadsapi::TerminateProcess;
        unsafe {
            let child_pid = child_pid as DWORD;
            let process_handle =
                OpenProcess(winapi::um::winnt::PROCESS_TERMINATE, FALSE, child_pid);
            let error_code = if process_handle.is_null() {
                let error_code = GetLastError();
                println!("无法打开进程。错误代码: {}", error_code);
                error_code
            } else {
                let error_code = if TerminateProcess(process_handle, 0) != FALSE {
                    println!("进程 {} 已终止.", child_pid);
                    0
                } else {
                    let error_code = GetLastError();
                    println!("无法终止进程。错误代码: {}", error_code);
                    error_code
                };
                CloseHandle(process_handle);
                error_code
            };
            return error_code as i32;
        }
    }
}
