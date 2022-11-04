mod currency;
mod money;
mod price;

pub use crate::price::Price;
pub use crate::money::Money;

pub use crate::currency::Currency;
pub type MinorUnit= u128;


#[cfg(doctest)]
#[macro_use]
extern crate doc_comment;


pub mod test {
    use crate::create_currency;

    create_currency!(SEK, 2);    
    create_currency!(EUR, 2);
}

#[cfg(doctest)]
doctest!("../README.md");
