use sha2::{Digest, Sha256};

pub fn encode(to_hash: String) -> String {
    let salt = "MYSALT";
    let to_hash = format!("{salt}{to_hash}");
    let mut hash = Sha256::new();
    hash.update(to_hash.as_bytes());
    let result = hash.finalize();

    hex::encode(result)
}
