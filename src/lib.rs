//! # lazier_static
//!
//! A thin wrapper around `std::sync::OnceLock` providing cached, static, lazy initialization. 
//!
//! ```rust
//! use lazier_static::*;
//! 
//! lazier_static! {
//!     fn hello_world() -> &str {
//!         "Hello, World!"
//!     }
//! }
//!
//! //...
//!
//! fn main() {
//!     println!("{}", hello_world());
//! }
//! ```
//!
//! ## License
//!
//! MIT OR Apache-2.0
//!
#[macro_export]
macro_rules! lazier_static {
    ($(#[$a:meta])* $v:vis fn $i:ident() -> &$t:ty $b:block $($rest:tt)*) => {
        __lazier_static_inner!($(#[$a])*, $v, $i, Box<&$t>, $t, Box::new($b));
        lazier_static!($($rest)*);
    };
    ($(#[$a:meta])* $v:vis fn $i:ident() -> $t:ty $b:block $($rest:tt)*) => {
        __lazier_static_inner!($(#[$a])*, $v, $i, $t, $t, $b);
        lazier_static!($($rest)*);
    };
    () => ()
}

#[macro_export]
macro_rules! __lazier_static_inner {
    ($(#[$a:meta])*, $v:vis, $i:ident, $ot:ty, $t:ty, $($b:tt)+) => {
        $(#[$a])*
        $v fn $i() -> &'static $t {
            static VALUE: std::sync::OnceLock<$ot> = std::sync::OnceLock::new();
            VALUE.get_or_init(|| $($b)+)
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn one_val_works() {
        lazier_static! {
            fn hi() -> i8 {
                10
            }
        }
        assert_eq!(hi(), &10_i8);
        assert_eq!(hi(), &10_i8);
    }
    #[test]
    fn one_str_works() {
        lazier_static! {
            fn food() -> &str {
                "yummy"
            }
        }
        assert_eq!(food(), "yummy");
        assert_eq!(food(), "yummy");
    }
    #[test]
    fn two_works() {
        lazier_static! {
            fn food() -> &str {
                "yummy"
            }
            fn hi() -> i8 {
                10
            }
        }
        assert_eq!(hi(), &10_i8);
        assert_eq!(hi(), &10_i8);
        assert_eq!(food(), "yummy");
        assert_eq!(food(), "yummy");
    }

    #[test]
    fn hashmap_works() {
        lazier_static! {
            fn magic() -> std::collections::HashMap<usize, usize> {
                [(2, 3), (4, 5)].into()
            }
        }
        let res = [(2, 3), (4, 5)].into();
        assert_eq!(magic(), &res);
        assert_eq!(magic(), &res);
    }
}
