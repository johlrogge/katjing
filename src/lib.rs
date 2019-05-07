//! Strongly typed money.
//!
//! Katjing is a money library that attempts to check as much as possible at compile time. If possible with zero run-time overhead.
//! If a compromise has to be made, chose the safer option. Katjing is mostly concerned with structural correctness and outsources
//! the details to other types. That is why [Money] is generic. Lastly  Katjing tries not to do surprising things!
//!
//! For example:
//!
//! * [Money] represents currency as phantom data allowing compile time checking of not mixing currencies while not using more memory than required to store the actual [Amount].
//! * [Money] is unsigned, can never go below zero, does not allow fractions and will return an error on overflows.
//! * [Money] is a representation of actual money in hand, when paying a [Cost] the money is consumed and returns the remaining [Cost] as change. This way, the same money cannot be used to pay more than one [Amount].
//! * All conversions between currencies have to be explicitly specified when needed.
//! * Katjing is experimental and has not been reviewed for production use, use at your own risk*
//!
//!  See the [book] for the full story.
//! 
//! [book]: ../../book/index.html

use core::fmt::Debug;
use core::marker::PhantomData;
use std::convert::{TryFrom, TryInto};

pub mod unit {
        use core::fmt::Debug;
        pub trait Unit
        where
                Self: Sized,
        {
                type C: super::Currency;
        }

        pub trait Main
        where
                Self: Unit + Debug,
        {
        }
        pub trait Cent
        where
                Self: Unit + Debug,
        {
        }
        pub trait Mill
        where
                Self: Unit + Debug,
        {
        }
}

/// Represents currency. Mainly to keep money in different currencies as separate types that cannot be used together without conversion
pub trait Currency
where
        Self: Sized,
{
        /// creates an instance of money in this currency
        fn create_money<V>(value: V) -> Money<V, Self>
        where
                V: AmountValue,
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
/// # #[macro_use] extern crate katjing;
/// # fn main () {
/// use katjing::prelude::*;
/// currencies![ (Eur Cent) ];
///
/// // Money uses no more memory than it's internal representation
/// assert_eq!(std::mem::size_of::<u32>(), std::mem::size_of::<Money<u32, Eur::Main>>());
/// }
/// ```
#[derive(Debug)]
pub struct Money<V, C>(V, PhantomData<C>)
where
        V: AmountValue,
        C: Currency;

impl<V, C> PartialEq<Money<V, C>> for Money<V, C>
where
        V: AmountValue,
        C: Currency,
{
        /// ```
        /// # #[macro_use] extern crate katjing;
        /// # fn main () {
        /// use katjing::prelude::*;
        /// currencies![(eur Cent)];
        ///
        /// let eur_12 = eur::create_money(12u32);
        /// assert_eq!(eur_12, eur_12);
        ///}
        ///```
        fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
        }
}

/// An abstract amount of money used for calculations
#[derive(Debug, Eq, Ord, PartialOrd)]
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
        /// # #[macro_use] extern crate katjing;
        /// # fn main () {
        /// use katjing::prelude::*;
        /// currencies![(eur Cent)];
        ///
        /// let eur_12 = eur::create_amount(12u32);
        /// let eur_13 = eur::create_amount(13u32);
        /// assert_eq!(eur_12, eur_12);
        /// assert_ne!(eur_13, eur_12);
        /// }
        /// ```
        fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
        }
}

/// Trait wrapping amounts. Used for implementing amount specializations like [Cost]
///
/// [Cost]: trait.Cost.html
pub trait WrappedAmount {
        type AV: AmountValue;
        type C: Currency;
        fn amount<'a>(&'a self) -> &'a Amount<Self::AV, Self::C>;
}

/// Represents a cost that can be payed
pub trait Cost
where
        Self: WrappedAmount,
{
        fn new(amount: Self::AV) -> Self;
}

/// Defines costs. You should declare different costs that are relevant such as *shipping*, *price*, or *tax*.
#[macro_export]
macro_rules! costs {
        (@cost ($c:ident $m:ident )) => {
                #[derive(Debug, Eq, PartialOrd, Ord)]
                struct $c<AV, C>($crate::Amount<AV, C>)
                where
                   AV: $crate::AmountValue,
                   C:  $crate::Currency;

                impl<AV, C> PartialEq for $c<AV, C>
                where
                   AV: $crate::AmountValue,
                   C: $crate::Currency,
                {
                        fn eq(&self, other: &$c<AV, C>) -> bool {
                                self.0 == other.0
                        }
                }

                impl<AVV, CC> $crate::WrappedAmount for $c<AVV, CC>
                where
                   AVV: $crate::AmountValue,
                   CC: $crate::Currency,
            {
                    type AV = AVV;
                    type C = CC;
                        fn amount(&self) -> &$crate::Amount<Self::AV, Self::C> {
                                &self.0
                        }
                }

                impl<AVV, CC> $crate::Cost for $c<AVV, CC>
                where
                   AVV: $crate::AmountValue,
                   CC: $crate::Currency,
            {
                        fn new(amount: Self::AV) -> Self {
                                $c(CC::create_amount(amount))
                        }
                }

            paste::item! {
                trait [< Create $c >] <C>
                where
                    C:$crate::Currency,
                {
                    fn [< create_ $m >] <AV> (value:AV) -> $c <AV, C>
                        where AV:$crate::AmountValue;
                }

                impl <C> [< Create $c >]<C> for C
                where
                    C:$crate::Currency,
                {
                    fn [< create_ $m >]<AV>  (value:AV) -> $c <AV, C>
                    where AV:$crate::AmountValue {
                        $c(C::create_amount(value))
                    }
                }
            }
        };

    ($(($c:ident $m:ident)),+) => ($(
        costs!(@cost ($c $m));
        )+)
}

/// The result of a [Take] operation
///
/// [Take]: trait.Take.html
struct Taken<MV, AV, C>
where
        MV: AmountValue,
        AV: AmountValue,
        C: Currency,
{
        /// the remaining money after the take
        pub remaining: Money<MV, C>,
        /// the taken amount. **taken may be less than asked for if there was not enough money to take from**
        pub taken: Amount<AV, C>,
}

/// The `take` operation. Used to remove an [Amount] from Money. Take is always safe even if [Amount] and
/// [Money] have different sizes of their representations.
///
/// [Amount]: struct.Amount.html
/// [Money]: struct.Money.html
fn take<MV, AV, C>(from: Money<MV, C>, amount: &Amount<AV, C>) -> Taken<MV, AV, C>
where
        C: Currency,
        AV: AmountValue + TryInto<MV> + TryFrom<MV>,
        MV: AmountValue + Debug,
        <AV as std::convert::TryFrom<MV>>::Error: Debug,
        <AV as std::convert::TryInto<MV>>::Error: Debug,
{
        use std::cmp::Ordering::*;
        let money_needed =
                amount.0.clone()
                        .try_into()
                        .unwrap_or_else(|_| MV::max_value());
        let remaining_money = from.0;
        match money_needed.cmp(&remaining_money) {
                Less => Taken {
                        remaining: C::create_money(
                                remaining_money
                                        .checked_sub(money_needed.clone())
                                        .expect("unexpected overflow"),
                        ),
                        taken: C::create_amount(
                                AV::try_from(money_needed).expect("should not be too large"),
                        ),
                },
                Equal => Taken {
                        remaining: C::create_money(MV::zero()),
                        taken: C::create_amount(
                                remaining_money.try_into().expect("should not be too large"),
                        ),
                },
                Greater => Taken {
                        remaining: C::create_money(MV::zero()),
                        taken: C::create_amount(
                                remaining_money.try_into().expect("should not be too large"),
                        ),
                },
        }
}

pub struct Change<M, A>
where
        M: Sized,
        A: Sized,
{
        pub money_back: M,
        pub left_to_pay: A,
}

/// Implement for payable things such as amounts
pub trait PayWith<MV>
where
        Self: Sized,
        MV: AmountValue,
{
        type C: Currency;

        /// consumes `with_money` and returns remaining money and left to pay after with_money has been
        /// deducted.
        #[must_use = "pay_with returns Change, it must be assigned"]
        fn pay_with(self, with_money: Money<MV, Self::C>) -> Change<Money<MV, Self::C>, Self>;
}

impl<CO, MV> PayWith<MV> for CO
where
        Self: Cost,
        <Self as WrappedAmount>::AV: AmountValue + TryInto<MV> + TryFrom<MV>,
        MV: AmountValue + TryInto<<Self as WrappedAmount>::AV>,
        <<Self as WrappedAmount>::AV as std::convert::TryFrom<MV>>::Error: Debug,
        <<Self as WrappedAmount>::AV as std::convert::TryInto<MV>>::Error: Debug,
{
        type C = <Self as WrappedAmount>::C;
        fn pay_with(self, with_money: Money<MV, Self::C>) -> Change<Money<MV, Self::C>, Self> {
                let Taken { remaining, taken } = take(with_money, self.amount());
                let left_to_pay = Self::new(
                        self.amount()
                                .0
                                .clone()
                                .checked_sub(taken.0)
                                .expect("overflow"),
                );

                Change {
                        money_back: remaining,
                        left_to_pay,
                }
        }
}

/// AmountValue is just a collection of traits needed to properly represent a monetary value.
/// It is implemented via a blanket implementation on all types that implement all the needed
/// traits. You should never need to implement AmountValue directly.
pub trait AmountValue
where
        Self: Clone + Sized + Debug + Zero + MaxValue + Eq + Ord + CheckedSub<Output = Self>,
{
}
/// Blanket implementation of AmountValue
impl<T> AmountValue for T where
        T: Clone + Sized + Debug + Zero + MaxValue + Eq + Ord + CheckedSub<Output = Self>
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

pub trait MaxValue {
        fn max_value() -> Self;
}
macro_rules! max_value_impl {
    ($($t: ty)*) => ($(
        impl crate::MaxValue for $t {
            #[inline]
            fn max_value() -> $t { return Self::max_value() }
        }
    )*)
}

max_value_impl![u8 u16 u32 u64 u128 usize];

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

#[macro_export]
macro_rules! currencies {
    (@unit Main) => {

    };
    (@unit $u:ident) => {

                #[derive(Debug)]
                pub struct $u {}
                impl $crate::unit::$u for $u {
                }

                impl $crate::unit::Unit for $u {
                    type C = $u;
                }

                impl $crate::Currency for $u {}

    };
    ($(($cur:ident $su:ident)),+) => {
        $(
            pub mod $cur {

                #[derive(Debug)]
                pub struct Main {}
                impl $crate::unit::Main for Main {
                }

                impl $crate::unit::Unit for Main {
                    type C = Main;
                }

                impl $crate::Currency for Main {}

                currencies!(@unit $su);

                pub fn create_money<AV:$crate::AmountValue>(value:AV) -> $crate::Money<AV, Main> {
                    use $crate::Currency;
                    Main::create_money(value)
                }

                pub fn create_amount<AV:$crate::AmountValue>(value:AV) -> $crate::Amount<AV, Main> {
                    use $crate::Currency;
                    Main::create_amount(value)
                }
        })+
    };
}

pub mod prelude {
        pub use crate::unit::{Cent, Main, Mill};
        pub use crate::{Amount, Change, Currency, Money, PayWith};
}

#[cfg(test)]
pub mod test {
        currencies!((eur Cent), (sek Cent));
        mod money {
                use super::eur;
                use crate::Money;
                #[test]
                fn create_from_currency() {
                        let eur_47 = eur::create_money(47u32);
                        let eur_11 = eur::create_money(11u32);

                        assert_eq!(eur_47, eur_47);
                        assert_ne!(eur_11, eur_47);
                }

                #[test]
                fn is_not_larger_than_value() {
                        assert!(std::mem::size_of::<Money<u32, eur::Main>>()
                                == std::mem::size_of::<u32>());
                }
        }

        mod amount {
                use super::eur;
                use crate::Amount;

                #[test]
                fn create_from_currency() {
                        let eur_47 = eur::create_amount(47u32);
                        let eur_11 = eur::create_amount(11u32);

                        assert_eq!(eur_47, eur_47);
                        assert_ne!(eur_47, eur_11);
                }

                #[test]
                fn is_not_larger_than_value() {
                        assert_eq!(
                                std::mem::size_of::<Amount<u32, eur::Main>>(),
                                std::mem::size_of::<u32>()
                        );
                }
        }

        mod take {

                mod money_and_amount_are_same_type {
                        use super::super::eur;
                        use crate::{take, Taken};

                        #[test]
                        fn take_full_amount_from_money() {
                                let money = eur::create_money(4711u32);
                                let amount = eur::create_amount(4711u32);
                                let Taken { remaining, taken } = take(money, &amount);
                                assert_eq!(remaining, eur::create_money(0u32));
                                assert_eq!(taken, eur::create_amount(4711u32));
                        }

                        #[test]
                        fn take_lower_amount_from_money() {
                                let money = eur::create_money(20u32);
                                let amount = eur::create_amount(15u32);
                                let Taken { remaining, taken } = take(money, &amount);
                                assert_eq!(remaining, eur::create_money(5u32));
                                assert_eq!(taken, eur::create_amount(15u32));
                        }

                        #[test]
                        fn take_higher_amount_from_money() {
                                let money = eur::create_money(15u32);
                                let amount = eur::create_amount(20u32);
                                let Taken { remaining, taken } = take(money, &amount);
                                assert_eq!(remaining, eur::create_money(0u32));
                                assert_eq!(taken, eur::create_amount(15u32));
                        }
                }
                mod money_is_a_larger_type_than_amount {
                        use super::super::eur;
                        use crate::{take, Taken};

                        #[test]
                        fn take_full_amount_from_money() {
                                let money = eur::create_money(4711u64);
                                let amount = eur::create_amount(4711u32);
                                let Taken { remaining, taken } = take(money, &amount);
                                assert_eq!(remaining, eur::create_money(0u64));
                                assert_eq!(taken, eur::create_amount(4711u32));
                        }

                        #[test]
                        fn take_lower_amount_from_money() {
                                let money = eur::create_money(20u64);
                                let amount = eur::create_amount(15u32);
                                let Taken { remaining, taken } = take(money, &amount);
                                assert_eq!(remaining, eur::create_money(5u64));
                                assert_eq!(taken, eur::create_amount(15u32));
                        }

                        #[test]
                        fn take_higher_amount_from_money() {
                                let money = eur::create_money(15u64);
                                let amount = eur::create_amount(20u32);
                                let Taken { remaining, taken } = take(money, &amount);
                                assert_eq!(remaining, eur::create_money(0u64));
                                assert_eq!(taken, eur::create_amount(15u32));
                        }

                        #[test]
                        fn money_has_larger_value_than_amount_can_represent() {
                                let money = eur::create_money(512u16);
                                let amount = eur::create_amount(128u8);
                                let Taken { remaining, taken } = take(money, &amount);

                                assert_eq!(remaining, eur::create_money(512u16 - 128));
                                assert_eq!(taken, eur::create_amount(128u8));
                        }
                }
                mod money_is_a_smaller_type_than_amount {
                        use super::super::eur;
                        use crate::{take, Taken};

                        #[test]
                        fn take_full_amount_from_money() {
                                let money = eur::create_money(4711u64);
                                let amount = eur::create_amount(4711u32);
                                let Taken { remaining, taken } = take(money, &amount);
                                assert_eq!(remaining, eur::create_money(0u64));
                                assert_eq!(taken, eur::create_amount(4711u32));
                        }

                        #[test]
                        fn take_lower_amount_from_money() {
                                let money = eur::create_money(20u64);
                                let amount = eur::create_amount(15u32);
                                let Taken { remaining, taken } = take(money, &amount);
                                assert_eq!(remaining, eur::create_money(5u64));
                                assert_eq!(taken, eur::create_amount(15u32));
                        }

                        #[test]
                        fn take_higher_amount_from_money() {
                                let money = eur::create_money(15u64);
                                let amount = eur::create_amount(20u32);
                                let Taken { remaining, taken } = take(money, &amount);
                                assert_eq!(remaining, eur::create_money(0u64));
                                assert_eq!(taken, eur::create_amount(15u32));
                        }

                        #[test]
                        fn amount_has_larger_value_than_money_can_represent() {
                                let money = eur::create_money(255u8);
                                let amount = eur::create_amount(512u16);
                                let Taken { remaining, taken } = take(money, &amount);
                                assert_eq!(remaining, eur::create_money(0u8));
                                assert_eq!(taken, eur::create_amount(255u16));
                        }
                }
        }

        mod pay_with {
                use super::eur;

                costs![(Price price)];

                mod money_is_same_type {
                        use super::*;
                        use crate::{Change, PayWith};
                        #[test]
                        fn pay_full_cost() {
                                let money = eur::create_money(4711u16);
                                let cost = eur::Main::create_price(4711u16);
                                let Change {
                                        money_back,
                                        left_to_pay,
                                } = cost.pay_with(money);
                                assert_eq!(money_back, eur::create_money(0u16));
                                assert_eq!(left_to_pay, eur::Main::create_price(0u16));
                        }

                        #[test]
                        fn pay_partial_cost() {
                                let money = eur::create_money(12u8);
                                let cost = eur::Main::create_price(24u8);
                                let Change {
                                        money_back,
                                        left_to_pay,
                                } = cost.pay_with(money);
                                assert_eq!(money_back, eur::create_money(0u8));
                                assert_eq!(left_to_pay, eur::Main::create_price(12u8));
                        }

                        #[test]
                        fn pay_more_than_cost() {
                                let money = eur::create_money(24u8);
                                let cost = eur::Main::create_price(12u8);
                                let Change {
                                        money_back,
                                        left_to_pay,
                                } = cost.pay_with(money);
                                assert_eq!(money_back, eur::create_money(12u8));
                                assert_eq!(left_to_pay, eur::Main::create_price(0u8));
                        }
                }
                mod money_is_a_smaller_type {
                        use super::*;
                        use crate::{Change, PayWith};
                        #[test]
                        fn pay_full_cost() {
                                let money = eur::create_money(128u8);
                                let cost = eur::Main::create_price(128u16);
                                let Change {
                                        money_back,
                                        left_to_pay,
                                } = cost.pay_with(money);

                                assert_eq!(money_back, eur::create_money(0u8));
                                assert_eq!(left_to_pay, eur::Main::create_price(0u16));
                        }

                        #[test]
                        fn pay_partial_cost() {
                                let money = eur::create_money(128u8);
                                let cost = eur::Main::create_price(255u16);
                                let Change {
                                        money_back,
                                        left_to_pay,
                                } = cost.pay_with(money);
                                assert_eq!(money_back, eur::create_money(0u8));
                                assert_eq!(left_to_pay, eur::Main::create_price(127u16));
                        }

                        #[test]
                        fn pay_more_than_cost() {
                                let money = eur::create_money(255u8);
                                let cost = eur::Main::create_price(128u16);
                                let Change {
                                        money_back,
                                        left_to_pay,
                                } = cost.pay_with(money);

                                assert_eq!(money_back, eur::create_money(255u8 - 128));
                                assert_eq!(left_to_pay, eur::Main::create_price(0u16));
                        }
                }
                mod money_is_a_larger_type {
                        use super::*;
                        use crate::{Change, PayWith};
                        #[test]
                        fn pay_full_cost() {
                                let money = eur::create_money(128u16);
                                let cost = eur::Main::create_price(128u8);
                                let Change {
                                        money_back,
                                        left_to_pay,
                                } = cost.pay_with(money);

                                assert_eq!(money_back, eur::create_money(0u16));
                                assert_eq!(left_to_pay, eur::Main::create_price(0u8));
                        }

                        #[test]
                        fn pay_partial_cost() {
                                let money = eur::create_money(128u16);
                                let cost = eur::Main::create_price(255u8);
                                let Change {
                                        money_back,
                                        left_to_pay,
                                } = cost.pay_with(money);
                                assert_eq!(money_back, eur::create_money(0u16));
                                assert_eq!(left_to_pay, eur::Main::create_price(127u8));
                        }

                        #[test]
                        fn pay_more_than_cost() {
                                let money = eur::create_money(4096u16);
                                let cost = eur::Main::create_price(255u8);
                                let Change {
                                        money_back,
                                        left_to_pay,
                                } = cost.pay_with(money);

                                assert_eq!(money_back, eur::create_money(4096u16 - 255));
                                assert_eq!(left_to_pay, eur::Main::create_price(0u8));
                        }
                }
        }
}
