mod vnet {
    use std::{
        collections::VecDeque,
        io::{Read, Result, Write},
        net::{SocketAddr, ToSocketAddrs},
        thread,
    };

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TcpListener {
        addr: SocketAddr,
        requests: VecDeque<&'static [u8]>,
    }

    impl TcpListener {
        pub fn bind<A>(addr: A) -> Result<TcpListener>
        where
            A: ToSocketAddrs,
        {
            let addr = addr.to_socket_addrs()?.next().ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "No address found",
            ))?;

            Ok(TcpListener {
                addr,
                requests: VecDeque::new(),
            })
        }

        pub fn local_addr(&self) -> Result<SocketAddr> {
            Ok(self.addr)
        }

        pub fn add_request(&mut self, request: &'static [u8]) {
            self.requests.push_back(request);
        }

        pub fn accept(&mut self) -> Result<(TcpStream, SocketAddr)> {
            loop {
                if let Some(request) = self.requests.pop_front() {
                    let stream = TcpStream {
                        read_data: request,
                        write_data: Vec::new(),
                        is_flushed: false,
                    };
                    return Ok((stream, self.addr));
                }
                thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TcpStream<'a> {
        read_data: &'a [u8],
        write_data: Vec<u8>,
        is_flushed: bool,
    }

    impl TcpStream<'_> {
        pub fn new() -> TcpStream<'static> {
            TcpStream {
                read_data: &[],
                write_data: Vec::new(),
                is_flushed: false,
            }
        }

        pub fn get_write_data(&self) -> Option<&[u8]> {
            if self.is_flushed {
                Some(&self.write_data)
            } else {
                None
            }
        }
    }

    impl Read for TcpStream<'_> {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let bytes = self.read_data;
            let len = bytes.len().min(buf.len());
            buf[..len].copy_from_slice(&bytes[..len]);
            Ok(len)
        }
    }

    impl Write for TcpStream<'_> {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            if self.is_flushed {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::WriteZero,
                    "Stream is flushed, cannot write",
                ));
            }
            self.write_data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            println!("Flushing data: {:?}", self.write_data);
            Ok(())
        }
    }
}

use std::io::Read;

use vnet::*;

fn main() {
    let mut l = TcpListener::bind("localhost:8080").unwrap();
    l.add_request(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n");
    l.add_request(
        b"POST /data HTTP/1.1\r\nHost: localhost\r\nContent-Type: text/plain\r\n\r\nHello",
    );

    loop {
        match l.accept() {
            Ok((mut stream, _)) => {
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();
                println!("Received request: {}", String::from_utf8_lossy(&buffer));
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}
