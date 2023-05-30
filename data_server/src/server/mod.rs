use std::{
    net::{
        TcpListener,
        TcpStream,
    },
    io::{
        Read,
        Write,
        BufReader,
        // BufWriter,
    },
    str::from_utf8,
};

pub struct Server {  }
impl Server {
    pub fn listen(port: u16, addr: &str) {
        println!("Server is active:\nhttp://{addr}:{port}");

        let listener = TcpListener::bind(format!("{addr}:{port}")).unwrap();
        listener.incoming().for_each(|stream| {
                let mut stream = stream.unwrap();
                let request = read_message_from(&mut stream);
                println!("{request}");
                if request.contains("/establish-data-server-connection") {
                    write_response(&mut stream, "We are connected :)");
                }
            });
    }
}

fn read_message_from(stream: &mut TcpStream) -> String {
    let mut stream_buffer: [u8; 8192] = [0u8; 8192];
    BufReader::new(stream).read(&mut stream_buffer).unwrap();
    return String::from(from_utf8(&stream_buffer).unwrap());
}

fn write_response(stream: &mut TcpStream, message: &str) {
    let message: &[u8] = message.as_bytes();
    stream.write_all(message).unwrap();
}