use std::{
    net::{
        TcpListener,
        Shutdown
    },
    fs,
    io::{
        BufReader,
        Read,
        Write,
    },
    thread,
    str::{
        from_utf8
    },
};
use serde::{
    Serialize,
    Deserialize,
};
pub struct HTTPServer {}
impl HTTPServer {
    pub fn listen(port: u16, addr: &str) {
        println!("{}\nServer is active:\n    http://{addr}:{port}",chrono::Local::now().to_rfc2822());
        let listener = TcpListener::bind(format!("{addr}:{port}")).unwrap();
        
        listener.incoming().for_each(|stream| {
        
            thread::spawn(|| {
                let mut stream = stream.unwrap();
                let mut stream_reader = BufReader::new(&mut stream);
                let mut stream_request: [u8; 8192] = [0; 8192];
                stream_reader.read(&mut stream_request).unwrap();
                let stream_request = from_utf8(&stream_request).unwrap();

                // Route Handling
                if stream_request.contains("GET / HTTP/1.1") {
                    let index_html = Website::get_site_resource("/index.html");
                    stream.write_all(index_html.as_bytes()).unwrap();

                } else if stream_request.contains("GET /css/style.css HTTP/1.1") {
                    let style_css = Website::get_site_resource("/css/style.css");
                    let response = format!(
                        "HTTP/1.1 200\r\nContent-Type: text/css\r\n\r\n{}",
                        style_css
                    );
                    stream.write_all(response.as_bytes()).unwrap();

                } else if stream_request.contains("GET /js/main.js HTTP/1.1") {
                    let main_js = Website::get_site_resource("/js/main.js");
                    let response = format!(
                        "HTTP/1.1 200\r\nContent-Type: text/javascript\r\n\r\n{}",
                        main_js
                    );
                    stream.write_all(response.as_bytes()).unwrap();

                } else if stream_request.contains("GET /income_data HTTP/1.1") {
                    stream.write_all(
                        format!("{}",
                            serde_json::to_string(
                                &IncomeData::get_from_file(".\\database\\income_data.json")
                            ).unwrap()
                        ).as_bytes()
                    ).unwrap();

                } else if stream_request.contains("POST /income_data/updated_hours HTTP/1.1") {
                    // /update_hours & /reset_hours will need to be changed to JSON formats.
                    // let mut hours_to_add: f32 = 0.0;
                    let message: f32 = Website::parse_post_request(stream_request).parse().unwrap();
                    let mut income_data = IncomeData::get_from_file(".\\database\\income_data.json");
                    income_data.add_hours(message);
                    income_data.save_to_file(".\\database\\income_data.json");

                    stream.write_all("{ \"status\": \"200\" }".as_bytes()).unwrap();
                    stream.shutdown(Shutdown::Both).unwrap();

                } else if stream_request.contains("POST /income_data/reset_hours HTTP/1.1") {
                    let mut income_data = IncomeData::get_from_file(".\\database\\income_data.json");
                    income_data.reset_hours();
                    income_data.save_to_file(".\\database\\income_data.json");
                    stream.write_all("{ \"status\": \"200\" }".as_bytes()).unwrap();
                    stream.shutdown(Shutdown::Both).unwrap();
                }

                stream.shutdown(Shutdown::Both).unwrap();
            });
        });
    }
}

#[derive(Debug)]
struct Website;
impl Website{
    fn get_site_resource(path: &str) -> String {
        let path = format!("./public/{path}");
        match fs::File::open(path) {
            Ok(mut file) => {
                let mut buf = [0u8; 8192];
                file.read(&mut buf).unwrap();
                let result = from_utf8(&buf).unwrap().trim_end_matches("\0");
                return String::from(result);
            },
            Err(e) => {
                println!("Something went wrong\n{:#?}\n", e);
                return String::new();
            }
        }
    }
    fn parse_post_request(request: &str) -> String {
        let request: Vec<&str> = request.split("\r\n\r\n").collect();
        if request[1].contains("\0") {
            let split: Vec<&str> = request[1].split("\0").collect();
            return String::from(split[0]);
        }
        return String::from(request[1]);
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct IncomeData {
    salary: f32,
    tax_rate: f32,
    taxed_salary: f32,
    hours_worked: f32,
    months_total: f32
}
impl IncomeData {
    fn get_from_file(path: &str) -> IncomeData {
        match fs::File::open(path) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                return serde_json::from_str(&contents).unwrap();
            },
            Err(e) => {
                println!("{e:#?}");
                return IncomeData {
                    salary: 0.0,
                    tax_rate: 0.0,
                    taxed_salary: 0.0,
                    hours_worked: 0.0,
                    months_total: 0.0,
                };
                
            }
        }
    }
    fn add_hours(&mut self, hours: f32) {
        self.hours_worked = (((self.hours_worked + hours) * 100.0).floor()) / 100.0;
        self.months_total = (((self.hours_worked * self.taxed_salary) * 100.0).floor()) / 100.0;

    }
    fn reset_hours(&mut self) {
        self.hours_worked = 0.0;
        self.months_total = 0.0;
    }
    fn save_to_file(&self, path: &str) {
        let json_data = serde_json::ser::to_string(self).unwrap();
        match fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path) {
                Ok(mut file) => {
                    file.set_len(json_data.len().try_into().unwrap()).unwrap();
                    file.write_all(&json_data.as_bytes()).unwrap();
                },
                Err(e) => println!("{e:#?}")
        };

    }
}