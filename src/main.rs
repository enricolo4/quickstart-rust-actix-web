use crate::init_server::init_rest_server;

mod init_server;

fn main() {
    init_rest_server().unwrap()
}