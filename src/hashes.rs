//!
//! The common hashing tools.
//!

use sha3::Digest;

pub fn keccak256(preimage: &[u8]) -> String {
    let hash_bytes = sha3::Keccak256::digest(preimage);
    hash_bytes
        .into_iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    #[test]
    fn ok() {
        assert_eq!(
            super::keccak256("zksync".as_bytes()),
            "0238fb1ab06c28c32885f9a4842207ac480c2467df26b6c58e201679628c5a5b"
        );
    }
}
