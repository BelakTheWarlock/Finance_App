use std::{
    process,
};

pub fn get_ip_address() -> String {
    // println!("Aquiring IP Address...")
    let mut ip_address: String = String::from("127.0.0.1");

    String::from_utf8(
        process::Command::new("hostname")
        .arg("-I")
        .output()
        .unwrap()
        .stdout
    ).unwrap()
    .lines()
    .for_each(|line| {
        ip_address = String::from(line.trim());
    });

    return ip_address;
}