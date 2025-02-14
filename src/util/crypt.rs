use rand::{prelude::*, rngs::OsRng};
use std::sync::LazyLock;

static ARGON2: LazyLock<argon2::Argon2> = LazyLock::new(|| {
    argon2::Argon2::default()
});

pub const SALT_SIZE: usize = 16;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Salt(pub [u8; SALT_SIZE]);

impl Salt {
    #[inline]
    pub fn os_random() -> Self {
        let mut salt = [0u8; SALT_SIZE];
        OsRng.fill_bytes(&mut salt);
        Self(salt)
    }
}

impl AsRef<[u8]> for Salt {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for Salt {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl std::ops::Deref for Salt {
    type Target = [u8; SALT_SIZE];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Salt {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<[u8; SALT_SIZE]> for Salt {
    #[inline]
    fn from(value: [u8; SALT_SIZE]) -> Self {
        Self(value)
    }
}

impl From<Salt> for [u8; SALT_SIZE] {
    #[inline]
    fn from(value: Salt) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Password {
    pub hash: [u8; 32],
    pub salt: Salt,
}

impl Password {
    #[inline]
    pub const fn new(hash: [u8; 32], salt: [u8; 16]) -> Self {
        Self {
            hash,
            salt: Salt(salt),
        }
    }

    #[inline]
    pub fn hash_password<P: AsRef<[u8]>>(password: P) -> Self {
        let salt = Salt::os_random();
        Self::hash_password_with(password.as_ref(), salt)
    }

    #[inline]
    pub fn hash_password_with<P: AsRef<[u8]>, S: Into<Salt>>(password: P, salt: S) -> Self {
        let salt: Salt = salt.into();
        let mut hash: [u8; 32] = [0; 32];
        ARGON2.hash_password_into(password.as_ref(), salt.as_ref(), &mut hash).expect("Failed to hash password.");
        Self {
            salt,
            hash,
        }
    }

    #[inline]
    pub fn compare<P: AsRef<[u8]>>(&self, password: P) -> bool {
        let mut hash: [u8; 32] = [0; 32];
        ARGON2.hash_password_into(password.as_ref(), self.salt.as_ref(), &mut hash).expect("Failed to hash password.");
        hash == self.hash
    }
    
    #[inline]
    pub fn hash(&self) -> &[u8] {
        &self.hash
    }

    #[inline]
    pub fn salt(&self) -> &Salt {
        &self.salt
    }
}

impl<P: AsRef<[u8]>> PartialEq<P> for Password {
    #[inline]
    fn eq(&self, other: &P) -> bool {
        self.compare(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn password_test() {
        let start = std::time::Instant::now();
        let hashed = Password::hash_password("hello, world");
        let first_hash_time = start.elapsed();
        let start = std::time::Instant::now();
        let result = hashed.compare(b"hello, world");
        let second_hash_time = start.elapsed();
        println!("Equal: {}", result);
        // 0x68
        // 0x65
        // 0x6c
        // 0x6c
        // 0x6f
        // 0x2c
        // 0x20
        // 0x77
        // 0x6f
        // 0x72
        // 0x6c
        // 0x64
        println!("Equal: {}", hashed == [
            0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x2c,
            0x20,
            0x77, 0x6f, 0x72, 0x6c, 0x64
        ]);
        println!(" First: {:.6}", first_hash_time.as_secs_f64());
        println!("Second: {:.6}", second_hash_time.as_secs_f64());
    }
}