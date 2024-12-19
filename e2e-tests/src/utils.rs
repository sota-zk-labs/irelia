use std::net::TcpListener;
use std::path::{Path, PathBuf};

use rand::Rng;

const MIN_PORT: u16 = 49_152;
const MAX_PORT: u16 = 65_535;
const MAX_TRIES: usize = 1000;
pub fn get_free_port() -> u16 {
    let mut rng = rand::thread_rng();
    for _ in 0..MAX_TRIES {
        let port = rng.gen_range(MIN_PORT..=MAX_PORT);
        if let Ok(listener) = TcpListener::bind(("127.0.0.1", port)) {
            return listener.local_addr().expect("No local addr").port();
        }
        // otherwise port is occupied
    }
    panic!("No free ports available");
}

pub fn get_repository_root() -> PathBuf {
    let manifest_path = Path::new(&env!("CARGO_MANIFEST_DIR"));
    let repository_root = manifest_path
        .parent()
        .expect("Failed to get parent directory of CARGO_MANIFEST_DIR");
    repository_root.to_path_buf()
}
