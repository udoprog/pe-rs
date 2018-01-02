#![feature(test)]
#![feature(iterator_step_by)]
#![feature(inclusive_range_syntax)]
#![feature(conservative_impl_trait)]
#![feature(io)]
#![feature(nonzero)]
#![feature(vec_remove_item)]
#![feature(drain_filter)]

mod comp_sieve;
mod hex_slice;
mod input;
mod permutation;
mod combination;
mod sieve;
mod prime;
mod digits;

extern crate core;
extern crate num;
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

modules!{
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
    p021,
    p022,
    p023,
    p024,
    p025,
    p026,
    p027,
    p028,
    p029,
    p030,
    p031,
    p032,
    p033,
    p034,
    p035,
    p036,
    p037,
    p038,
    p039,
    p040,
    p041,
    p042,
    p043,
    p044,
    p045,
    p046,
    p047,
    p048,
    p049,
    p050,
    p051,
    p052,
    p053,
    p054,
    p055,
    p056,
    p057,
    p058,
    p059,
    p060,
    p061,
    p062,
    p067,
}
