use std::fs;

use sysinfo::{System, SystemExt };

fn read_logo(file: &str) -> String {
    let contents = fs::read_to_string(file)
    .expect("can't find logo");
    contents
}

fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i32.pow(decimals) as f64;
    (x * y).round() / y
}

fn replace_data(logo: &str) -> String{
    let mut sys = System::new();
    sys.refresh_all();
    let na = "Can't obtain info";

    let logo = logo.replace("<name>", &sys.name().expect(na));
    let logo = logo.replace("<kernel_version>", &sys.kernel_version().expect(na));
    let logo = logo.replace("<os_version>", &sys.os_version().expect(na));
    let logo = logo.replace("<cpu_len>", &sys.cpus().len().to_string());

    let mut m_gb = (sys.total_memory()/(1024*1024*1024)).to_string();
    if m_gb == "15" {  m_gb = "16".to_owned() }
    let logo = logo.replace("<total_memory>", &m_gb);

    let logo = logo.replace("<load.one>", &(round(sys.load_average().one, 2).to_string()));
    let logo = logo.replace("<load.five>", &(round(sys.load_average().five, 2).to_string()));
    let logo = logo.replace("<load.fifteen>", &(round(sys.load_average().fifteen,2).to_string()));
    logo.to_owned()
}

fn main() {
    let mut logo = read_logo(&(std::env::var("HOME").expect("CAN'T FIND HOME VARIABLE")+"/.config/rfetch/message.txt"));
    logo = replace_data(&logo);
    print!("{}", logo);
}
