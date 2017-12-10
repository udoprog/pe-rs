#![feature(test)]
#![feature(iterator_step_by)]
#![feature(inclusive_range_syntax)]
#![feature(conservative_impl_trait)]

mod hex_slice;
mod sieve;

extern crate sha2;
extern crate test;

/// Function used to hash answers.
///
/// Uses dynamic dispatch to avoid excessive monomorphization.
fn hash(value: &::std::fmt::Display) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::default();
    hasher.input(value.to_string().as_bytes());
    let output = hasher.result();
    hex_slice::HexSlice::new(&output[..]).to_string()
}

#[macro_export]
macro_rules! problem {
    (tests => [$($test:tt)*]; $($rest:tt)*) => {
        #[cfg(test)]
        mod benches {
            #[allow(unused)]
            use super::*;

            problem!(@bench $($test)*);
        }

        problem!(@test $($test)*);

        #[allow(unused)]
        pub fn run_all(name: &str, spoil: bool) {
            println!("# {}", name);
            problem!(@print spoil $($test)*);
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
            assert_eq!($crate::hash(&$test).as_str(), $exp);
        }

        problem!(@test $($rest)*);
    };

    (@test) => {};

    (@print $spoil:ident $name:ident => ($test:expr, $exp:expr), $($rest:tt)*) => {
        println!("{:10} => {} = {:?}", stringify!($name), stringify!($test), $test);
        problem!(@print $spoil $($rest)*)
    };

    (@print $spoil:ident $name:ident => {$test:expr, $exp:expr}, $($rest:tt)*) => {
        let _r = $test;

        if $spoil {
            println!(
                "{:10} => {} = {:?} <{}>",
                stringify!($name),
                stringify!($test),
                _r,
                $crate::hash(&_r)
            );
        } else {
            println!(
                "{:10} => {} = <{}>",
                stringify!($name),
                stringify!($test),
                $crate::hash(&_r)
            );
        }

        problem!(@print $spoil $($rest)*)
    };

    (@print $spoil:ident) => {};

    () => {
    };
}

macro_rules! modules {
    ($($mod:ident,)*) => {
        $(mod $mod;)*

        pub fn run_all(spoil: bool, filters: &[&str]) {
            println!("WARNING: SPOILERS ARE PRINTED!");
            $(
            if filters.iter().all(|f| stringify!($mod).contains(f)) {
                self::$mod::run_all(stringify!($mod), spoil);
            }
            )*
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
    p012,
    p013,
    p014,
    p015,
    p016,
    p017,
    p018,
    p019,
    p020,
    p067,
];
