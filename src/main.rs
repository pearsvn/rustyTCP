// Author: Alfie Pearson
// Date: 28.08.2026
// Project: Implementing TCP in Rust

use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf: [u8; 2] = [0, 255];
    let nbytes = nic.recv(&mut buf[..])?;
    eprintln!("read {} bytes: {:?}", nbytes, &buf[..nbytes]);
    Ok(())
}
