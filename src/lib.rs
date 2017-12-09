#![feature(test)]
#![feature(iterator_step_by)]
#![feature(inclusive_range_syntax)]

#[cfg(test)]
mod hex_slice;
mod sieve;

extern crate sha2;
extern crate test;

#[macro_export]
macro_rules! problem {
    (tests => [$($test:tt)*]; $($rest:tt)*) => {
        #[cfg(test)]
        mod benches {
            #[allow(unused)]
            use super::*;

            problem!(@bench $($test)*);
        }

        #[cfg(test)]
        mod tests {
            #[allow(unused)]
            use super::*;

            problem!(@test $($test)*);
        }

        pub fn run_all(name: &str) {
            println!("# {}", name);
            problem!(@print $($test)*);
        }

        problem!($($rest)*);
    };

    (@bench $name:ident => ($test:expr, $exp:expr), $($rest:tt)*) => {
        #[bench]
        fn $name(b: &mut ::test::Bencher) {
            b.iter(|| $test);
        }

        problem!(@bench $($rest)*);
    };

    (@bench $name:ident => {$test:expr, $exp:expr}, $($rest:tt)*) => {
        #[bench]
        fn $name(b: &mut ::test::Bencher) {
            b.iter(|| $test);
        }

        problem!(@bench $($rest)*);
    };

    (@bench) => {};

    (@test $name:ident => ($test:expr, $exp:expr), $($rest:tt)*) => {
        #[test]
        fn $name() {
            assert_eq!($test, $exp);
        }

        problem!(@test $($rest)*);
    };

    (@test $name:ident => {$test:expr, $exp:expr}, $($rest:tt)*) => {
        #[test]
        fn $name() {
            use $crate::sha2::Digest;
            let mut hasher = $crate::sha2::Sha256::default();
            hasher.input($test.to_string().as_bytes());
            let output = hasher.result();
            let hex = $crate::hex_slice::HexSlice::new(&output[..]).to_string();
            assert_eq!(hex.as_str(), $exp);
        }

        problem!(@test $($rest)*);
    };

    (@test) => {};

    (@print $name:ident => ($test:expr, $exp:expr), $($rest:tt)*) => {
        println!("{:10} => {} = {:?}", stringify!($name), stringify!($test), $test);
        problem!(@print $($rest)*)
    };

    (@print $name:ident => {$test:expr, $exp:expr}, $($rest:tt)*) => {
        println!("{:10} => {} = {:?}", stringify!($name), stringify!($test), $test);
        problem!(@print $($rest)*)
    };

    (@print) => {};

    () => {
    };
}

macro_rules! modules {
    ($($mod:ident,)*) => {
        $(mod $mod;)*

        pub fn run_all() {
            $(self::$mod::run_all(stringify!($mod));)*
        }
    }
}

modules![
    p001,
    p002,
    p003,
    p004,
    p005,
    p006,
    p007,
    p008,
    p009,
    p010,
    p011,
    p067,
];
