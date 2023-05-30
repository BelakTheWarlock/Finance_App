use std::{
    process,
};

pub fn get_ip_address() -> String {
    // println!("Acquiring IP Address...");

    let mut ip_address: String = String::from("127.0.0.1");

    String::from_utf8(
        process::Command::new("ipconfig")
        .output()
        .unwrap()
        .stdout
    ).unwrap()
    .lines()
    .for_each(|line| {

        if line.contains("IPv4") {
            ip_address = String::from(line.split(": ").last().unwrap());
            return;
        }
    });

    return ip_address;
}