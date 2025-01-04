use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use windows::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};

fn main() {
    unsafe {
        let snapshot: HANDLE = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
            .expect("Failed to create process snapshot.");
        if snapshot == INVALID_HANDLE_VALUE {
            eprintln!("Failed to create process snapshot.");
            return;
        }

        let mut entry = PROCESSENTRY32W::default();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

        while Process32NextW(snapshot, &mut entry).is_ok() {
            let process_name = OsString::from_wide(&entry.szExeFile)
                .to_string_lossy()
                .into_owned();

            let process_id = entry.th32ProcessID;
            let thread_count = entry.cntThreads;

            println!("{}|{}|{}\n", process_name, process_id, thread_count);
        }

        let _ = CloseHandle(snapshot);
    }
}
