extern crate hyper;

use std::io::Write;
use std::sync::Mutex;
use std::sync::mpsc::{channel, Sender};
use hyper::server::{Handler, Server, Request, Response};

struct SenderHandler {
    sender: Mutex<Sender<&'static str>>,
}

impl Handler for SenderHandler {
    fn handle(&self, req: Request, mut res: Response) {
        println!("Recieved Request");
        self.sender.lock().unwrap().send("start").unwrap();
        let body = b"Pong";

        let mut res = res.start().unwrap();
        res.write_all(body).unwrap();
    }
}


fn main() {
    let (tx, rx) = channel();

    Server::http("127.0.0.1:9000")
        .unwrap()
        .handle(SenderHandler { sender: Mutex::new(tx) })
        .unwrap();
}
