use std::fs;
use std::io::{self, Write};

fn read_line(prompt : &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()  // Often you want to trim whitespace
}

fn main(){
    println!("Welcome to the .desktop generator script. \nWritten in Rust. Version 0.1 \n\nAll files are written to /home/ben/.local/share/applications/\n\nEnter your Web App details below:\n");
    let name = read_line("Name: ");
    let comment = read_line("Description: ");
    let url = read_line("URL: ");
    let icon = read_line("Icon: ");
    


    let file_content = format!("[Desktop Entry]
Version=1.0
Name={name} (Web App)
Comment={comment}
Exec=chromium-browser --app={url} --class={name}
Icon={icon}
Terminal=false
Type=Application
Categories=Network;WebBrowser;
StartupWMClass={name}
StartupNotify=true
");

    print!("{}", file_content);
    fs::write(format!("/home/ben/.local/share/applications/{name}.desktop"), file_content);
}
