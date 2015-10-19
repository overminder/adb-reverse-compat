use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use std::io::{self, Read, Write};

pub enum SimpleFwdOp {
    Connect(u32),
    Connected(u32),
    Data(u32, Vec<u8>),
    Disconnect(u32),
    Disconnected(u32),  // Really want FIN ACK?
}

/*
impl SimpleFwdOp {
    pub fn write_to<A: Write + Sized>(&self, os: &mut A) -> io::Result<()> {
        use SimpleFwdOp::*;
        match self {
            &Connect(id) => {
                try!(os.write_byte(1));
                try!(os.write_all(1));
            }
        }
    }
}
*/

pub fn read_exact<A: Read + Sized>(stream: &mut A, buf: &mut [u8]) -> io::Result<()> {
    stream.take(buf.len() as u64)
        .read(buf)
        .map(|_| ())
}

pub fn read_frame<A: Read + Sized>(stream: &mut A) -> io::Result<Vec<u8>> {
    let length = try!(stream.read_u32::<BigEndian>());
    let mut buf = vec![0; length as usize]; 
    try!(read_exact(stream, &mut buf));
    Ok(buf)
}

pub fn write_frame<A: Write + Sized>(stream: &mut A, buf: &[u8]) -> io::Result<()> {
    try!(stream.write_u32::<BigEndian>(buf.len() as u32));
    try!(stream.write_all(buf));
    stream.flush()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn check_write_read(bs: &[u8]) {
        let mut buf = vec![];
        write_frame(&mut buf, &bs).unwrap();
        let bs2 = read_frame(&mut Cursor::new(buf)).unwrap();
        assert_eq!(bs2, bs);
    }

    #[test]
    fn test_write_read() {
        check_write_read(&[]);
        check_write_read(b"a");
        check_write_read(b"asdf");
        check_write_read(b"asdf5");
    }
}

