pub use hexmacros::*;

#[macro_export]
macro_rules! pipeline {
    ($input:expr => $($pipe:expr) =>+) => {
        (|piped| {
            $(
                let piped = ($pipe)(piped);
            )*
            piped
        })($input)
    };
}

pub use crate::pipeline;

#[macro_export]
macro_rules! hash_password {
    ($lit:literal) => {
        $crate::util::crypt::HashedPassword::hash_password($lit)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pipeline_test() {
        
        fn step1(input: i32) -> i32 {
            input + 55
        }
        fn step2(input: i32) -> u8 {
            input as u8
        }

        fn step3(input: u8) -> String {
            const HEX_CHARS: [char; 16] = [
                '0',
                '1',
                '2',
                '3',
                '4',
                '5',
                '6',
                '7',
                '8',
                '9',
                'A',
                'B',
                'C',
                'D',
                'E',
                'F'
            ];
            let hex1 = HEX_CHARS[(input >> 4 & 0xF) as usize];
            let hex2 = HEX_CHARS[(input & 0xF) as usize];
            format!("{hex1}{hex2}")
        }
        let result = pipeline!(200 => step1 => step2 => step3 => |s: String| s.to_lowercase());
        debug_assert_eq!(result, "ff");
        let result = pipeline!(100 => |n: i32| n + 1 => |n: i32| n * 2);
        debug_assert_eq!(result, 202);

    }

    #[test]
    fn hash_password_test() {
        prototype!{my_table!{
            [1, 2, 3, 4]
            ["hello, world"]
            [one two three]
            [env!("CARGO_PKG_NAME")]
        }}
        foreach!(std::println!(
            ("Hello, world!")
            ("This is a test")
            ("Does this work? {}", env!("CARGO_PKG_NAME"))
        ));
    }
}