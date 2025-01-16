
use std::sync::LazyLock;

static ARGON2: LazyLock<argon2::Argon2> = LazyLock::new(|| {
    argon2::Argon2::default()
});

pub struct HashedPassword {
    salt: [u8; 32],
    hash: [u8; 32],
}

impl HashedPassword {
    pub const fn new(salt: [u8; 32], hash: [u8; 32]) -> Self {
        Self {
            salt,
            hash,
        }
    }

    pub fn hash_password<P: AsRef<[u8]>>(password: P) -> Self {
        let salt: [u8; 32] = rand::random();
        Self::hash_password_with(password.as_ref(), salt)
    }

    pub fn hash_password_with<P: AsRef<[u8]>>(password: P, salt: [u8; 32]) -> Self {
        let mut hash: [u8; 32] = [0; 32];
        ARGON2.hash_password_into(password.as_ref(), &salt, &mut hash).expect("Failed to hash password.");
        Self {
            salt,
            hash,
        }
    }

    pub fn compare<P: AsRef<[u8]>>(&self, password: P) -> bool {
        let mut hash: [u8; 32] = [0; 32];
        ARGON2.hash_password_into(password.as_ref(), &self.salt, &mut hash).expect("Failed to hash password.");
        hash == self.hash
    }

    pub fn salt(&self) -> &[u8] {
        &self.salt
    }
    
    pub fn hash(&self) -> &[u8] {
        &self.hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn password_test() {
        let start = std::time::Instant::now();
        let hashed = HashedPassword::hash_password(b"hello, world");
        let first_hash_time = start.elapsed();
        let start = std::time::Instant::now();
        println!("Equal: {}", hashed.compare(b"hello, world"));
        let second_hash_time = start.elapsed();
        println!(" First: {:.6}", first_hash_time.as_secs_f64());
        println!("Second: {:.6}", second_hash_time.as_secs_f64());
    }
}