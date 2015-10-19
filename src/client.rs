extern crate byteorder;
extern crate adb_reverse;

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
struct ForwardParams {
    listen_port: u16,
}

fn establish_initial_pipe(port: u16) -> io::Result<(ForwardParams, TcpStream)> {
    let listener = try!(TcpListener::bind(("127.0.0.1", port)));
    println!("[Client] Initial pipe bound: {:?}", listener);

    for stream in listener.incoming() {
        println!("[Client] Incoming pipe: {:?}", stream);
        return read_fwd_params(try!(stream));
    }

    Err(io::Error::new(io::ErrorKind::Other, "Listener closed unexpectedly"))
}

fn read_fwd_params<A: Read + Write + Sized>(mut stream: A) -> io::Result<(ForwardParams, A)> {
    let listen_port = try!(stream.read_u16::<BigEndian>());

    Ok((ForwardParams { listen_port: listen_port, }, stream))
}

fn start_fwd_server(params: ForwardParams) -> io::Result<()> {
    let listener = try!(TcpListener::bind(("127.0.0.1", params.listen_port)));
    println!("[Client] start_fwd_server: Bound: {:?}", listener);
    Ok(())
}

fn main() {
    let (fwd_params, pipe_conn) = establish_initial_pipe(9876).unwrap();
    println!("[Client] fwd_params = {:?}", fwd_params);
    start_fwd_server(fwd_params).unwrap();
}

