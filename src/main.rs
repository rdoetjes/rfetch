use std::{fs, collections::HashMap};
use local_ip_address::local_ip;
use sysinfo::{System, SystemExt };
use std::path::PathBuf;

fn read_logo(file: &str) -> String {
    let contents = fs::read_to_string(file)
    .expect("can't find logo");
    contents
}

fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i32.pow(decimals) as f64;
    (x * y).round() / y
}

fn replace_data(mut logo: String) -> String{
    let mut sys = System::new();
    sys.refresh_all();
    let na: String = "Can't obtain info".to_string();    
    let m_gb = (sys.total_memory()/(1000*1000*1024)).to_string(); 
    let ip = local_ip().expect("Could not get ip");

    // hashmap where the key is the token in the template and the value, is the value with what the token will be replaced
    let replacement = HashMap::from([
        ("<name>", sys.name().unwrap_or(na.clone())),
        ("<kernel_version>", sys.kernel_version().unwrap_or(na.clone())),
        ("<os_version>", sys.os_version().unwrap_or(na.clone())),
        ("<cpu_len>", sys.cpus().len().to_string()),
        ("<total_memory>", m_gb),
        ("<load.one>", (round(sys.load_average().one, 2).to_string())),
        ("<load.five>", (round(sys.load_average().five, 2).to_string())),
        ("<load.fifteen>", (round(sys.load_average().fifteen, 2).to_string())),
        ("<ip>", ip.to_string()),
        ("<r>", "\x1b[31m".to_string()),
        ("<g>", "\x1b[32m".to_string()),
        ("<y>", "\x1b[33m".to_string()),
        ("<b>", "\x1b[34m".to_string()),
        ("<m>", "\x1b[35m".to_string()),
        ("<c>", "\x1b[36m".to_string()),
        ("<w>", "\x1b[37m".to_string()),
        ]);
    
    for kv in replacement {
       logo = logo.replace(kv.0, &kv.1);
    }
    logo
}

fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| panic!("HOME environment variable not set"));
    PathBuf::from(home).join(".config").join("rfetch").join("message.txt")
}

fn main() {
    let config_path = get_config_path();
    let logo_template = read_logo(config_path.to_str().unwrap());
    //jus overwrite the logo pointer, we don't need the unfilled template
    let logo = replace_data(logo_template);
    print!("{}", logo);
}
