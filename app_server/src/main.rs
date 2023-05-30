mod http_server;
mod net_assist;
use http_server::HTTPServer;
fn main() {
    HTTPServer::listen(
        80,
        net_assist::get_ip_address().as_str()
    );
}
