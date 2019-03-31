//! Strong typed money.
//!
//! Katjing is a money library that attempts to check as much as possible at compile time. If possible with zero runtime overhead. If a compromize has to be made, chose the safer option. Katjing is mostly concerned with structural correctness and outsources the details to other types. That is why [Money] is generic. Lastly  Katjing tries not to do surprising things!
//!
//! For example:
//!
//! * [Money] represents currency as Phantom data allowing compiletime checking of not mixing currencies while not using more memory than reqired to store the actual [Amount].
//! * [Money] is unsigned, can never go below zero, does not allow fractions and will fail on overflows.
//! * [Money] is a representation of actual money in hand, when paying an [Amount] the money is consumed and returns the remaning [Amount] as change. This way, the same money cannot be used to pay more then one [Amount].
//! * Katjing separates [Money] and [Amount]s. [Money] is always rounded to be representable as real [Money] while an [Amount] representing something like an interest can have a fractional part. This part will be represented as rounding if needed.
//! * All conversions have to be explictly specified when needed.
//!
//! *Katjing is experimental and has not been reviewed for production use, use at your own risk*
//!
//! [Money]: struct.Money.html
//! [Amount]: struct.Amount.html

#[doc(html_playground_url = "https://play.rust-lang.org/")]

use core::fmt::Debug;
use core::marker::PhantomData;


/// Represents currency. Mainly to keep money in different currencies as separate types that cannot be used together without conversion
pub trait Currency
where
        Self: Sized,
{
        /// creates an instance of money in this currency
        fn create<V>(value: V) -> Money<V, Self>
        where
                V: MoneyValue,
        {
                Money(value, PhantomData::<Self>)
        }
}

/// A representation of money. Money has a value and a currency. The currency is PhantomData meaning that has no size and is only relevant in compiletime.
///
/// ```
/// use katjing::{Money, Currency, currencies};
/// currencies!(Eur);
///
/// // Money uses no more memory than it's internal representation
/// assert_eq!(std::mem::size_of::<i32>(), std::mem::size_of::<Money<i32, Eur>>());
/// ```
#[derive(Debug)]
pub struct Money<V: MoneyValue, C: Currency>(V, PhantomData<C>);

impl<V, C> PartialEq<Money<V, C>> for Money<V, C>
where
        V: MoneyValue,
        C: Currency,
{
        /// ```
        /// use katjing::{Money, Currency, currencies};
        /// currencies!(Eur);
        ///
        /// let eur_12 = Eur::create(12);
        /// assert_eq!(eur_12, eur_12);
        ///```
        fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
        }
}

/// An amount represents something that can be paid. Like a Fee, Price, Shipping or Amortization
pub struct Amount<M, V, C> (V, PhantomData<C>, PhantomData<M>)
where
    M:MoneyValue + Into<V>,
    V:AmountValue<M>,
    C:Currency ;


/// MoneyValue is just a collection of traits needed to properly represent a monetary value. It is implemented via a blanket implementation on all types that implement all the needed traits. You should never need to implement MoneyValue directly.
pub trait MoneyValue
where
        Self: Sized + Debug + PartialEq<Self>,
{
}
/// Blanket implementation of MoneyValue
impl<T> MoneyValue for T where T: Sized + Debug + PartialEq<T> {}


/// AmountValue is just a collection of traits needed to properly represent a monetary value. It is implemented via a blanket implementation on all typs that implement all the needed traits. You should never need to implement MoneyValue directly.
pub trait AmountValue<MV>
where
    MV:MoneyValue + Into<Self>,
    Self: MoneyValue + Into<MV> {
}

/// Blanket implementation of AmountValue
impl<T, MV> AmountValue<MV> for T where
    T: MoneyValue + Into<MV>,
    MV: MoneyValue + Into<T>,
{
}

#[macro_export]
macro_rules! currencies {
    ($($cur:ident),+) => {
        $(
        #[derive(Debug)]
        pub struct $cur();
        impl Currency for $cur {})+
    };
}

#[cfg(test)]
pub mod test {
        use crate::{Currency, Money};
        currencies!(Eur, Sek);
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
