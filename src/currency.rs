pub trait Currency: std::fmt::Debug {
    const ALPHABETIC_CODE:&'static str;
    const MINOR_UNIT:u8;
}

#[macro_export]
macro_rules! create_currency {
    ($name:ident, $minor_unit_decimals:expr) => (
        #[derive(Debug, PartialEq, PartialOrd)]
        pub struct $name();
        impl $crate::Currency for $name {
            const ALPHABETIC_CODE:&'static str = stringify!($name);
            const MINOR_UNIT:u8 = 10u8.pow($minor_unit_decimals);
        }
    )
}
