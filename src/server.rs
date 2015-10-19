extern crate byteorder;
extern crate adb_reverse;

use std::thread;
use std::process::{Child, Command, Stdio};
use std::io::{self, Read, Write};
use std::net::{TcpStream};
use byteorder::{BigEndian, WriteBytesExt};

fn spawn_client() -> io::Result<Child> {
    Command::new("adb")
        .arg("shell")
        .arg("/data/local/tmp/adb-reverse-client")
        .spawn()
}

fn adb_forward_pipe_port(pipe_port: u16) -> io::Result<Child> {
    let tcp_arg = format!("tcp:{}", pipe_port);
    Command::new("adb")
        .arg("forward")
        .arg(&tcp_arg)
        .arg(&tcp_arg)
        .spawn()
}

fn connect_client(pipe_port: u16, fwd_port: u16) -> io::Result<TcpStream> {
    let mut stream = try!(TcpStream::connect(("localhost", pipe_port)));
    println!("[Server] connect_client: connected to {:?}", stream);
    try!(stream.write_u16::<BigEndian>(fwd_port));
    try!(stream.flush());
    Ok(stream)
}

fn main() {
    // Just keep it running.
    let mut client_proc = spawn_client().unwrap();
    let pipe_port = 9876;
    adb_forward_pipe_port(pipe_port).unwrap().wait().unwrap();
    thread::sleep_ms(500);  // XXX: Read from client_proc instead
    let pipe_conn = connect_client(9876, 9999).unwrap();
    client_proc.wait().unwrap();
}

