/// ```no_run
/// macro_rules! num_impl {
///     ($type:ty) => {
///         // impl here
///     };
/// }
/// for_each_int_type!(num_impl)
/// // or
/// for_each_int_type!(num_impl;signed)
/// // or
/// for_each_int_type!(num_impl;unsigned)
/// ```
#[macro_export]
macro_rules! for_each_int_type {
    ($macro:path) => {
        $crate::for_each_int_type!($macro;unsigned);
        $crate::for_each_int_type!($macro;signed);
    };
    ($macro:path;unsigned) => {
        $macro!{usize}
        $macro!{u128}
        $macro!{u64}
        $macro!{u32}
        $macro!{u16}
        $macro!{u8}
    };
    ($macro:path;signed) => {
        $macro!{isize}
        $macro!{i128}
        $macro!{i64}
        $macro!{i32}
        $macro!{i16}
        $macro!{i8}
    }
}
#[macro_export]
macro_rules! pipeline {
    ($input:expr => $($pipe:expr),+) => {
        (|piped| {
            $(
                let piped = ($pipe)(piped);
            )*
            piped
        })($input)
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
        let result = pipeline!(200 => step1, step2, step3);
        println!("{result}");
    }
}