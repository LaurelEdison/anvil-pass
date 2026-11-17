use std::{
    fs::File,
    io::{self, Read},
    net::{IpAddr, SocketAddr, TcpListener},
    ops::RangeInclusive,
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};

const PORT_RANGE: RangeInclusive<u16> = 1024..=49151;

pub struct KdbxFile {
    pub path: PathBuf,
    pub hash: String,
}

fn hash_file(path: &Path) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(hex::encode(hasher.finalize()))
}

fn find_available_port(ip: &IpAddr) -> u16 {
    for port in PORT_RANGE {
        if can_listen_on_port(ip, &port) {
            return port;
        }
    }
    return 0;
}

fn can_listen_on_port(ip: &IpAddr, port: &u16) -> bool {
    let addr = SocketAddr::new(*ip, *port);
    TcpListener::bind(addr).is_ok()
}
