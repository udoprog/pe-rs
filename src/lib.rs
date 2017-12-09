#![feature(test)]
#![feature(iterator_step_by)]
#![feature(inclusive_range_syntax)]

mod sieve;

extern crate test;

#[macro_export]
macro_rules! problem {
    (tests => [$($name:ident => ($exp:expr, $test:expr),)*]; $($rest:tt)*) => {
        #[cfg(test)]
        mod benches {
            #[allow(unused)]
            use super::*;

            $(
            #[bench]
            fn $name(b: &mut ::test::Bencher) {
                b.iter(|| $test);
            }
            )*
        }

        #[cfg(test)]
        mod tests {
            #[allow(unused)]
            use super::*;

            $(
            #[test]
            fn $name() {
                assert_eq!($test, $exp);
            }
            )*
        }

        pub fn run_all(name: &str) {
            println!("# {}", name);
            $(println!("{:10} => {} = {:?}", stringify!($name), stringify!($test), $test);)*
        }

        problem!($($rest)*);
    };

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
    p067,
];
