//! This module implements the http server support for our application.

use std::net::ToSocketAddrs;

use hyper::server::Server;

use app::Pencil;


/// Run the `Pencil` application.
pub fn run_server<A: ToSocketAddrs>(application: Pencil, addr: A, threads: usize) {
    let server = Server::http(addr).unwrap();
    let _guard = server.handle_threads(application, threads).unwrap();
}
