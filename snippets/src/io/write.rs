use std::io::{self, ErrorKind, Read, Write};

pub fn copy<R : Read + ?Sized , W: Write + ?Sized>(reader: &mut R, writer: &mut W) -> io::Result<u64>
{
    let mut buf = [0; 10];
    let mut written = 0;
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };

        writer.write_all(&buf[..len]);
        written += len as u64;
    }
}


#[test]
fn copy_test() {
    let mut reader = "This is the buffer content".as_bytes();
    let mut writer : Vec<u8> = Vec::new();
    copy(&mut reader, &mut writer);

    let utf8_str = String::from_utf8(writer).unwrap();
    assert_eq!(utf8_str, "This is the buffer content");

}