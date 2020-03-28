use std::process::exit;

use clap::{App, Arg};

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("gateway-id")
        .version(VERSION)
        .author("Orne Brocaar <info@brocaar.com>")
        .about("Print the Gateway ID")
        .arg(
            Arg::with_name("command_url")
                .short("c")
                .long("command-url")
                .help("ZMQ command URL")
                .default_value("ipc:///tmp/concentratord_command")
                .takes_value(true),
        )
        .get_matches();
    let command_url = matches.value_of_lossy("command_url").unwrap();

    // create new zmq REQ socket
    let zmq_ctx = zmq::Context::new();
    let zmq_sock = zmq_ctx.socket(zmq::REQ).expect("new ZMQ socket error");
    zmq_sock.connect(&command_url).expect("ZMQ connect error");

    // send 'gateway_id' command with empty payload
    zmq_sock.send("gateway_id", zmq::SNDMORE).unwrap();
    zmq_sock.send("", 0).unwrap();

    // set poller so that we can timeout after 100ms
    let mut items = [zmq_sock.as_poll_item(zmq::POLLIN)];
    zmq::poll(&mut items, 100).unwrap();
    if !items[0].is_readable() {
        println!("could not read gateway_id");
        exit(1);
    }

    // read 'gateway_id' response
    let gateway_id = zmq_sock.recv_bytes(0).unwrap();
    println!("{}", hex::encode(gateway_id));
    exit(0);
}
