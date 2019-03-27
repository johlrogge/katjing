//! Katjing is a library for representing money.
//! The philosophy is to be able to lean on the type system for as much as possible with as little runtime overhead as possible. Zero cost abstractions if you will.
//!
//! For example, Currency is important, it makes no sense to add 2 EUR to 5 USD. Then you would just have 2 EUR and 5 USD. You could add upp the total _value_ in SEK if you wanted to but that would require conversion, which leads to the second principle: No surprises.

use core::fmt::Debug;
use core::marker::PhantomData;

/// Value is just a collection of traits needed to properly represent a monetayr value. It implemented via a blanket implementation on all types that implement all the needed traits. You should never need to implement Value directly.
pub trait Value
where
        Self: Sized + Debug + PartialEq<Self>,
{
}
/// Blanket implementation of Value
impl<T> Value for T where T: Sized + Debug + PartialEq<T> {}

/// Represents currency. Mainly to keep money in different currencies as separate types that cannot be used together without conversion
pub trait Currency
where
        Self: Sized,
{
        /// creates an instance of money in this currency
        fn create<V>(value: V) -> Money<V, Self>
        where
                V: Value,
        {
                Money(value, PhantomData::<Self>)
        }
}

/// A representation of money. Money has a value and a currency. The currency is PhantomData meaning that has no size and is only relevant in compiletime.
///
/// ```
/// use katjing::{Money, Currency};
/// #[derive(Debug)]
/// struct Eur();
/// impl Currency for Eur {}
///
/// // Money uses no more memory than it's internal representation
/// assert_eq!(std::mem::size_of::<i32>(), std::mem::size_of::<Money<i32, Eur>>());
/// ```
#[derive(Debug)]
pub struct Money<V: Value, C: Currency>(V, PhantomData<C>);

impl<V, C> PartialEq<Money<V, C>> for Money<V, C>
where
        V: Value,
        C: Currency,
{
        /// ```
        /// use katjing::{Money, Currency};
        /// #[derive(Debug)]
        /// struct Eur();
        /// impl Currency for Eur {}
        ///
        /// let eur_12 = Eur::create(12);
        /// assert_eq!(eur_12, eur_12);
        ///```
        fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
        }
}

#[cfg(test)]
pub mod test {
        use crate::{Currency, Money};
        #[derive(Debug)]
        struct Eur();
        impl Currency for Eur {}

        #[test]
        fn create_money_from_currency() {
                let eur_47 = Eur::create(47);
                let eur_11 = Eur::create(11);
                assert_eq!(eur_47, eur_47);
                assert_ne!(eur_11, eur_47);
        }

        #[test]
        fn money_is_not_larger_than_value() {
                assert!(std::mem::size_of::<Money<i32, Eur>>() == std::mem::size_of::<i32>());
        }

}
