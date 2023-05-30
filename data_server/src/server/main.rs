mod server;
mod net_assist;
use server::Server;
fn main() {
    Server::listen(
        8080,
        net_assist::get_ip_address().as_str()
    );
}