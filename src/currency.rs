pub trait Currency: std::fmt::Debug {
    const ALPHABETIC_CODE: ::iso_currency::Currency;
    fn minor_unit() -> u16 {
        Self::ALPHABETIC_CODE.subunit_fraction().unwrap_or_else(||1)
    }
}

#[macro_export]
macro_rules! create_currency {
    ($name:ident, $minor_unit_decimals:expr) => (
        #[derive(Debug, PartialEq, PartialOrd)]
        pub struct $name();
        impl $crate::Currency for $name {
            const ALPHABETIC_CODE: ::iso_currency::Currency = ::iso_currency::Currency::$name;
        }
    )
}
