# helfsteal
Simple Data Stealer

Hi All,

I published basic data stealer malware with Rust. FOR EDUCATIONAL PURPOSES. You can use it for Red Team operations or training and improve.

Anti-VM & Sandbox (Sandbox Process Detection and VM Processes Detection)

Anti-Debug (CheckRemoteDebuggerPresent)

Malware looking some file extensions (pdf,doc,docx and etc) and creating zip under "AppData/Local/Temp/". Zip name is "sensfiles.zip" then sending to C2 server. Because we are looking for sensitive files and we have to be faster because we will be shadow after the operation. So persistence not necessary for us. Some APT groups collect sensitive files on the victim system for use of blackmail, espionage, etc.
And  I published simple Python source code for server-side.


References: 

https://anal.school/ -- For optimization and code quality. He is best Rust developer ever.

https://crates.io/crates/process_list

https://docs.rs/zip/0.5.12/zip/write/struct.ZipWriter.html




Chesapeake Ripper

Vive la guerre éternelle
