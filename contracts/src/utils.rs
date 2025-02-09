// use sha2::{Digest, Sha256};

// pub fn hash_data(data: &[u8]) -> String {
//     let mut hasher = Sha256::new();
//     hasher.update(data);
//     format!("{:x}", hasher.finalize())
// }

// pub fn validate_address(address: &Address) -> bool {
//     !address.is_zero()
// }