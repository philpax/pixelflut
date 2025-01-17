use std::io::{self, BufRead, Write};

use std::net::TcpStream;

pub struct Pixelflut {
    write: TcpStream,
    read: io::BufReader<TcpStream>,
}

impl Pixelflut {
    pub fn connect(host: &str) -> io::Result<Pixelflut> {
        let stream = TcpStream::connect(host)?;
        let read = io::BufReader::new(stream.try_clone()?);
        Ok(Self {
            write: stream,
            read,
        })
    }

    pub fn size(&mut self) -> io::Result<(u32, u32)> {
        writeln!(self.write, "SIZE")?;

        let mut line = String::new();
        self.read.read_line(&mut line)?;

        let mut iter = line
            .split_ascii_whitespace()
            .skip(1)
            .map(|v| v.parse::<u32>().expect("expected integer for size"));
        Ok((iter.next().unwrap(), iter.next().unwrap()))
    }

    pub fn read(&mut self, x: u32, y: u32) -> io::Result<(u8, u8, u8)> {
        writeln!(self.write, "PX {} {}", x, y)?;

        let mut line = String::new();
        self.read.read_line(&mut line)?;

        let colour_string = line
            .split_ascii_whitespace()
            .nth(3)
            .expect("expected colour at end of PX");

        Ok((
            u8::from_str_radix(&colour_string[0..2], 16).unwrap(),
            u8::from_str_radix(&colour_string[2..4], 16).unwrap(),
            u8::from_str_radix(&colour_string[4..6], 16).unwrap(),
        ))
    }

    pub fn write(&mut self, x: u32, y: u32, color: (u8, u8, u8)) -> io::Result<()> {
        let hex = format!("{:02x}{:02x}{:02x}", color.0, color.1, color.2);
        writeln!(self.write, "PX {} {} {}", x, y, hex)
    }
}
