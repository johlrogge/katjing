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
        fn create_money<V>(value: V) -> Money<V, Self>
        where
                V: MoneyValue,
        {
                Money(value, PhantomData::<Self>)
        }

        fn create_amount<V>(value: V) -> Amount<V, Self>
        where
                V: AmountValue,
        {
                Amount(value, PhantomData::<Self>)
        }
}

/// A representation of money. Money has a value and a currency. The currency is PhantomData meaning that has no size and is only relevant in compiletime.
///
/// ```
/// use katjing::{Money, Currency, currencies};
/// currencies!(Eur);
///
/// // Money uses no more memory than it's internal representation
/// assert_eq!(std::mem::size_of::<u32>(), std::mem::size_of::<Money<u32, Eur>>());
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
        /// let eur_12 = Eur::create_money(12u32);
        /// assert_eq!(eur_12, eur_12);
        ///```
        fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
        }
}

/// An amount represents something that can be paid. Like a Fee, Price, Shipping or Amortization
#[derive(Debug)]
pub struct Amount<AV, C>(AV, PhantomData<C>)
where
        AV: AmountValue,
        C: Currency;

impl<V, C> PartialEq<Amount<V, C>> for Amount<V, C>
where
        V: AmountValue,
        C: Currency,
{
        /// ```
        /// use katjing::{Amount, Currency, currencies};
        /// currencies!(Eur);
        ///
        /// let eur_12 = Eur::create_amount(12u32);
        /// let eur_13 = Eur::create_amount(13u32);
        /// assert_eq!(eur_12, eur_12);
        /// assert_ne!(eur_13, eur_12);
        /// ```
        fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
        }
}

pub struct Taken<MV, AV, C>
where
        MV: MoneyValue,
        AV: AmountValue,
        C: Currency,
{
        pub remaining: Money<MV, C>,
        pub taken: Amount<AV, C>,
}

pub trait Take<MV, C>
where
        MV: MoneyValue,
        Self: Sized,
        C: Currency,
{
        fn take<AV>(self, amount: &Amount<AV, C>) -> Taken<MV, AV, C>
        where
                AV: AmountValue + Into<MV> + From<MV>;
}

impl<MV, C> Take<MV, C> for Money<MV, C>
where
        C: Currency,
        MV: MoneyValue,
{
        fn take<AV>(self, amount: &Amount<AV, C>) -> Taken<MV, AV, C>
        where
                AV: AmountValue + Into<MV> + From<MV>,
        {
                use std::cmp::Ordering::*;
                let money_needed = amount.0.clone().into();
                let remaining_money = self.0;
                match money_needed.cmp(&remaining_money) {
                        Less => Taken {
                                remaining: C::create_money(
                                        remaining_money
                                                .checked_sub(money_needed.clone())
                                                .expect("unexpected overflow"),
                                ),
                                taken: C::create_amount(money_needed.into()),
                        },
                        Equal => Taken {
                                remaining: C::create_money(MV::zero()),
                                taken: C::create_amount(remaining_money.into()),
                        },
                        Greater => Taken {
                                remaining: C::create_money(MV::zero()),
                                taken: C::create_amount(remaining_money.into()),
                        },
                }
        }
}

/// Implement for payable things such as amounts
pub trait Pay<C>
where
        C: Currency,
{
        /// consumes `with_money` and returns remaining money and left to pay after with_money has been deducted.
        fn pay<MV>(self, with_money: Money<MV, C>) -> (Money<MV, C>, Self)
        where
                MV: MoneyValue;
}

impl<AV, C> Pay<C> for Amount<AV, C>
where
        AV: AmountValue,
        C: Currency,
{
        fn pay<MV>(self, with_money: Money<MV, C>) -> (Money<MV, C>, Self)
        where
                MV: MoneyValue,
        {
                (with_money, self)
        }
}

/// MoneyValue is just a collection of traits needed to properly represent a monetary value. It is implemented via a blanket implementation on all types that implement all the needed traits. You should never need to implement MoneyValue directly.
pub trait MoneyValue
where
        Self: Clone + Sized + Debug + Zero + Eq + Ord + CheckedSub<Output = Self>,
{
}
/// Blanket implementation of MoneyValue
impl<T> MoneyValue for T where T: Clone + Sized + Debug + Zero + Eq + Ord + CheckedSub<Output = Self>
{}

/// AmountValue is just a collection of traits needed to properly represent a monetary value. It is implemented via a blanket implementation on all typs that implement all the needed traits. You should never need to implement MoneyValue directly.
pub trait AmountValue
where
        Self: MoneyValue,
{
}

pub trait Zero {
        fn zero() -> Self;
}

macro_rules! zero_impl {
    ($($t: ty)*) => ($(
        impl crate::Zero for $t {
            #[inline]
            fn zero() -> $t { 0 }
        }
        )*)
}

zero_impl![u8 u16 u32 u64 u128 usize];

pub trait CheckedSub<Rhs = Self> {
        type Output;
        fn checked_sub(self, rhs: Rhs) -> Option<Self::Output>;
}

macro_rules! checked_sub_impl {
    ($($t: ty)*) => ($(
        impl crate::CheckedSub for $t {
            type Output = $t;
            #[inline]
            fn checked_sub(self, other: $t) -> Option<$t> { self.checked_sub(other) }
        }
        )*)
}

checked_sub_impl!(u8 u16 u32 u64 u128 usize);

/// Blanket implementation of AmountValue
impl<T> AmountValue for T where T: MoneyValue + Clone {}

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
        use crate::Currency;
        currencies!(Eur, Sek);
        mod money {
                use super::Eur;
                use crate::{Currency, Money};
                #[test]
                fn create_from_currency() {
                        let eur_47 = Eur::create_money(47u32);
                        let eur_11 = Eur::create_money(11u32);

                        assert_eq!(eur_47, eur_47);
                        assert_ne!(eur_11, eur_47);
                }

                #[test]
                fn is_not_larger_than_value() {
                        assert!(std::mem::size_of::<Money<u32, Eur>>()
                                == std::mem::size_of::<u32>());
                }
        }

        mod amount {
                use super::Eur;
                use crate::{Amount, Currency};

                #[test]
                fn create_from_currency() {
                        let eur_47 = Eur::create_amount(47u32);
                        let eur_11 = Eur::create_amount(11u32);

                        assert_eq!(eur_47, eur_47);
                        assert_ne!(eur_47, eur_11);
                }

                #[test]
                fn is_not_larger_than_value() {
                        assert_eq!(
                                std::mem::size_of::<Amount<u32, Eur>>(),
                                std::mem::size_of::<u32>()
                        );
                }
        }

        mod take {
                use super::Eur;
                use crate::{Currency, Take, Taken};

                #[test]
                fn take_full_amount_from_money() {
                        let money = Eur::create_money(4711u32);
                        let amount = Eur::create_amount(4711u32);
                        let Taken { remaining, taken } = money.take(&amount);
                        assert_eq!(remaining, Eur::create_money(0u32));
                        assert_eq!(taken, Eur::create_amount(4711u32));
                }

                #[test]
                fn take_lower_amount_from_money() {
                        let money = Eur::create_money(20u32);
                        let amount = Eur::create_amount(15u32);
                        let Taken { remaining, taken } = money.take(&amount);
                        assert_eq!(remaining, Eur::create_money(5u32));
                        assert_eq!(taken, Eur::create_amount(15u32));
                }

                #[test]
                fn take_higher_amount_from_money() {
                        let money = Eur::create_money(15u32);
                        let amount = Eur::create_amount(20u32);
                        let Taken { remaining, taken } = money.take(&amount);
                        assert_eq!(remaining, Eur::create_money(0u32));
                        assert_eq!(taken, Eur::create_amount(15u32));
                }
        }
}
