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
//! # Examples
//! Some code will probably illustrate the above much better.
//!
//! ## Currencies
//! Katjing treats currencies as separate types. That means you cannot mix amounts of different currencies.
//! Let's define some currencies and create some money:
//!
//! ```
//! # #[macro_use] extern crate katjing;
//! # fn main () {
//! use katjing::{Currency, Main, Cent, Mill};
//! currencies![(IDR Main), (EUR Cent), (KWD Mill)];
//! let some_idr = IDR::create_money(18u8);
//! let some_eur = EUR::create_money(40_000u128);
//! let some_kwd = KWD::create_money(64_000u32);
//! # }
//! ```
//! As you can see you can create money of different types, and you create them from a currency.
//! The currency is just phantomdata and is only relevant during compile time and allows rust to
//! make sure that you don't do nonsensical things by mistake [^adding]
//!
//! When you specify a currency with the `currencies!` macro you also specify the minimal representable unit of the currency.
//! The minimal representable unit can be
//! <dl>
//! <dt>Main</dt><dd>Has no subunit as is the case with the Indonesian Rupiah <em>IDR</em> in the example</dd>
//! <dt>Cent</dt><dd>Subunit is 1/100 of the main unit. This is the most common subunit type used</dd>
//! <dt>Mill</dt><dd>Subunit is 1/1000 of the main unit as is the case with the Kuwaiti Dinar <em>KWD</em> used in the example</dd>
//! </dl>
//!
//! The subunit is abstract and is not concerned with the actual names of the subunits. For instance: SEK would declare
//! `Cent` even though the actual name of the subunit is *öre*.
//!
//! We will look more into subunits later but for now, that is what they mean.
//!
//! But what good is wealth if you can't spend it on anything?
//!
//! [^adding]: like adding 2 SEK to 10 USD, Then you would just have 2 SEK and 10 USD unless you BUY
//!            USD for your SEK which is a different operation entirely)
//!
//! ## Costs
//! Just like currencies katjing allows you to define costs. A cost is something that can be covered (payed), like
//! a Price, Shipping, VAT, or Interest. We will get back to how to calculate VAT and interests in a bit.
//!
//! ```
//! # #[macro_use] extern crate katjing;
//! # use katjing::{Currency, Cent};
//! use katjing::Cost;
//! # currencies![(EUR Cent), (SEK Cent), (USD Cent)];
//! costs![(Shipping shipping), (Price price)];
//! fn main () {
//!   let shipping_eur = EUR::create_shipping(1u8);
//!   let shipping_usd = USD::create_shipping(2u8);
//!   let price_eur = EUR::create_price(100u16);
//!   let price_usd = USD::create_price(128u16);
//! }
//! ```
//!
//! As you may notice you can choose *storage type* for your costs and your money. You may not expect shipping in € to be a very large amount
//! so you may choose a small type to represent shipping while you would choose something bigger to represent national debt.
//! You may also notice that all values are unsigned. This is also by design. Negative money does not exist, that is only for calculation. Katjing uses types instead of signs as we will see.
//! Lastly, costs and money are created from a currency. **You cannot mix currencies**
//! From now on we will assume the above costs and currencies are defined.
//!
//! As mentioned initially Katjing tries to prevent as many errors as possible at compile time. Here are a few examples we can demonstrate with what we know:
//!
//! ```compile_fail
//! # #[macro_use] extern crate katjing;
//! # use katjing::Currency;
//! # use katjing::Cost;
//! # currencies![EUR, SEK, USD];
//! # costs![(Shipping shipping), (Price price)];
//! # fn main () {
//!     let shipping_eur:Shipping<u8, USD> = EUR::create_shipping(1u8);
//! # }
//! ```
//! *You cannot assign a cost to a cost different currency.*
//!
//! ```compile_fail
//! # #[macro_use] extern crate katjing;
//! # use katjing::Currency;
//! # use katjing::Cost;
//! # currencies![EUR, SEK, USD];
//! # costs![(Shipping shipping), (Price price)];
//! # fn main () {
//!     let shipping_eur:Shipping<u8, EUR> = EUR::create_price(1u8);
//! # }
//! ```
//! *You cannot assign a cost to a cost of a different type.*
//!
//! ```compile_fail
//! # #[macro_use] extern crate katjing;
//! # use katjing::Currency;
//! # currencies![EUR, SEK, USD];
//! # fn main () {
//!     let money_usd:Money<u8, USD> = EUR::create_money(1u8);
//! # }
//! ```
//! *You cannot assign money to money of a different currency.*
//!
//! ```
//! # #[macro_use] extern crate katjing;
//! # use katjing::{Currency, Cent};
//! # currencies![(EUR Cent), (SEK Cent), (USD Cent)];
//! # fn main () {
//!     EUR::create_money(1u8);
//! # }
//! ```
//! *Warns if you don't assign created money to something.*
//!
//! ## Paying costs
//!
//! While creating costs and money can be fun. Let's use it for something:
//!
//! ```
//! # #[macro_use] extern crate katjing;
//! # use katjing::Currency;
//! use katjing::{PayWith, Change, Cent};
//! // define costs and currencies
//! # costs![(Price price), (Shipping shipping)];
//! # currencies![(EUR Cent), (USD Cent)];
//! # fn main() {
//! let shipping = EUR::create_shipping(12u8);
//! let money = EUR::create_money(1_000u16);
//!
//! let Change{money_back, left_to_pay} = shipping.pay_with(money);
//! assert_eq!(money_back, EUR::create_money(1_000u16-12));
//! assert_eq!(left_to_pay, EUR::create_shipping(0u8));
//! # }
//! ```
//!
//! There are two important things to note here:
//!
//! 1. `pay_with` returns `Change` containing any remaining money and cost after the payment
//! 2. `pay_with` consumes `money`, it cannot be used again.
//!
//! Let's try to use money after the payment:
//!
//!
//! ```compile_fail
//! # #[macro_use] extern crate katjing;
//! # use katjing::{Currency, Change, PayWith};
//! # costs![(Price price), (Shipping shipping)];
//! # currencies![EUR, USD];
//! # fn main() {
//! let shipping = EUR::create_shipping(12u8);
//! let price = EUR::create_price(1_000u16);
//! let money = EUR::create_money(1_012u16);
//!
//! let Change{ money_back, left_to_pay } = price.pay_with(money);
//! let Change{ money_back, left_to_pay } = shipping.pay_with(money); // <- fails: money has already been used to pay price with
//! # }
//! ```
//! *`money` has moved when paying `price`, it cannot be used again. This prevents paying the cost with money we don't have.*
//!
//! Let's fix the code above:
//!
//! ```
//! # #[macro_use] extern crate katjing;
//! # use katjing::{Currency, Change, PayWith, Cent};
//! # costs![(Price price), (Shipping shipping)];
//! # currencies![(EUR Cent), (USD Cent)];
//! # fn main() {
//! let shipping = EUR::create_shipping(12u8);
//! let price = EUR::create_price(1_000u16);
//! let money = EUR::create_money(1_014u16);
//!
//! let Change{ money_back:money, left_to_pay:price } = price.pay_with(money);
//! let Change{ money_back:money, left_to_pay:shipping } = shipping.pay_with(money);
//!
//! assert_eq!( money, EUR::create_money(2u16));
//! assert_eq!( price, EUR::create_price(0u16));
//! assert_eq!( shipping, EUR::create_shipping(0u8));
//! # }
//! ```
//!
//! [Money]: struct.Money.html
//! [Amount]: struct.Amount.html
//! [Cost]: trait.Cost.html

use core::fmt::Debug;
use core::marker::PhantomData;
use std::convert::{TryFrom, TryInto};

pub trait SubUnit
where
        Self: Sized,
{
}

#[derive(Debug)]
pub struct Main();
#[derive(Debug)]
pub struct Cent();
#[derive(Debug)]
pub struct Mill();

impl SubUnit for Main {}
impl SubUnit for Cent {}
impl SubUnit for Mill {}

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
/// use katjing::{Money, Currency, Cent, currencies};
/// currencies![ (Eur Cent) ];
///
/// // Money uses no more memory than it's internal representation
/// assert_eq!(std::mem::size_of::<u32>(), std::mem::size_of::<Money<u32, Eur>>());
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
        /// use katjing::{Money, Currency, Cent, currencies};
        /// currencies![(Eur Cent)];
        ///
        /// let eur_12 = Eur::create_money(12u32);
        /// assert_eq!(eur_12, eur_12);
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
        /// use katjing::{Amount, Currency, Cent, currencies};
        /// currencies![(Eur Cent)];
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

/// Trait wrapping amounts. Used for implementing amount specializations like [Cost]
///
/// [Cost]: trait.Cost.html
pub trait WrappedAmount<AV>
where
        AV: AmountValue,
{
        type C: Currency;
        fn amount<'a>(&'a self) -> &'a Amount<AV, Self::C>;
}

/// Represents a cost that can be payed
pub trait Cost<AV>
where
        Self: WrappedAmount<AV>,
        AV: AmountValue,
{
        type C: Currency;
        fn new(amount: AV) -> Self;
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

                impl<AV, CC> $crate::WrappedAmount<AV> for $c<AV, CC>
                where
                   AV: $crate::AmountValue,
                   CC: $crate::Currency,
            {
                    type C = CC;
                        fn amount(&self) -> &$crate::Amount<AV, Self::C> {
                                &self.0
                        }
                }

                impl<AV, CC> $crate::Cost<AV> for $c<AV, CC>
                where
                   AV: $crate::AmountValue,
                   CC: $crate::Currency,
            {
                    type C = CC;
                        fn new(amount: AV) -> Self {
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
pub struct Taken<MV, AV, C>
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
pub fn take<MV, AV, C>(from: Money<MV, C>, amount: &Amount<AV, C>) -> Taken<MV, AV, C>
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
pub trait PayWith<MV, AV>
where
        Self: Sized,
        MV: AmountValue,
        AV: AmountValue,
{
        type C: Currency;
        /// consumes `with_money` and returns remaining money and left to pay after with_money has been
        /// deducted.
        #[must_use = "pay_with returns Change, it must be assigned"]
        fn pay_with(self, with_money: Money<MV, Self::C>) -> Change<Money<MV, Self::C>, Self>;
}

impl<CO, MV, AV> PayWith<MV, AV> for CO
where
        Self: Cost<AV>,
        AV: AmountValue + TryInto<MV> + TryFrom<MV>,
        MV: AmountValue + TryInto<AV>,
        <AV as std::convert::TryFrom<MV>>::Error: Debug,
        <AV as std::convert::TryInto<MV>>::Error: Debug,
{
        type C = <Self as WrappedAmount<AV>>::C;
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
    ($(($cur:ident $su:ident)),+) => {
        $(
        #[derive(Debug)]
        pub struct $cur($su);
        impl $crate::Currency for $cur {})+
    };
}

#[cfg(test)]
pub mod test {
        use crate::Cent;
        currencies!((Eur Cent), (Sek Cent));
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

                mod money_and_amount_are_same_type {
                        use super::super::Eur;
                        use crate::{take, Currency, Taken};

                        #[test]
                        fn take_full_amount_from_money() {
                                let money = Eur::create_money(4711u32);
                                let amount = Eur::create_amount(4711u32);
                                let Taken { remaining, taken } = take(money, &amount);
                                assert_eq!(remaining, Eur::create_money(0u32));
                                assert_eq!(taken, Eur::create_amount(4711u32));
                        }

                        #[test]
                        fn take_lower_amount_from_money() {
                                let money = Eur::create_money(20u32);
                                let amount = Eur::create_amount(15u32);
                                let Taken { remaining, taken } = take(money, &amount);
                                assert_eq!(remaining, Eur::create_money(5u32));
                                assert_eq!(taken, Eur::create_amount(15u32));
                        }

                        #[test]
                        fn take_higher_amount_from_money() {
                                let money = Eur::create_money(15u32);
                                let amount = Eur::create_amount(20u32);
                                let Taken { remaining, taken } = take(money, &amount);
                                assert_eq!(remaining, Eur::create_money(0u32));
                                assert_eq!(taken, Eur::create_amount(15u32));
                        }
                }
                mod money_is_a_larger_type_than_amount {
                        use super::super::Eur;
                        use crate::{take, Currency, Taken};

                        #[test]
                        fn take_full_amount_from_money() {
                                let money = Eur::create_money(4711u64);
                                let amount = Eur::create_amount(4711u32);
                                let Taken { remaining, taken } = take(money, &amount);
                                assert_eq!(remaining, Eur::create_money(0u64));
                                assert_eq!(taken, Eur::create_amount(4711u32));
                        }

                        #[test]
                        fn take_lower_amount_from_money() {
                                let money = Eur::create_money(20u64);
                                let amount = Eur::create_amount(15u32);
                                let Taken { remaining, taken } = take(money, &amount);
                                assert_eq!(remaining, Eur::create_money(5u64));
                                assert_eq!(taken, Eur::create_amount(15u32));
                        }

                        #[test]
                        fn take_higher_amount_from_money() {
                                let money = Eur::create_money(15u64);
                                let amount = Eur::create_amount(20u32);
                                let Taken { remaining, taken } = take(money, &amount);
                                assert_eq!(remaining, Eur::create_money(0u64));
                                assert_eq!(taken, Eur::create_amount(15u32));
                        }

                        #[test]
                        fn money_has_larger_value_than_amount_can_represent() {
                                let money = Eur::create_money(512u16);
                                let amount = Eur::create_amount(128u8);
                                let Taken { remaining, taken } = take(money, &amount);

                                assert_eq!(remaining, Eur::create_money(512u16 - 128));
                                assert_eq!(taken, Eur::create_amount(128u8));
                        }
                }
                mod money_is_a_smaller_type_than_amount {
                        use super::super::Eur;
                        use crate::{take, Currency, Taken};

                        #[test]
                        fn take_full_amount_from_money() {
                                let money = Eur::create_money(4711u64);
                                let amount = Eur::create_amount(4711u32);
                                let Taken { remaining, taken } = take(money, &amount);
                                assert_eq!(remaining, Eur::create_money(0u64));
                                assert_eq!(taken, Eur::create_amount(4711u32));
                        }

                        #[test]
                        fn take_lower_amount_from_money() {
                                let money = Eur::create_money(20u64);
                                let amount = Eur::create_amount(15u32);
                                let Taken { remaining, taken } = take(money, &amount);
                                assert_eq!(remaining, Eur::create_money(5u64));
                                assert_eq!(taken, Eur::create_amount(15u32));
                        }

                        #[test]
                        fn take_higher_amount_from_money() {
                                let money = Eur::create_money(15u64);
                                let amount = Eur::create_amount(20u32);
                                let Taken { remaining, taken } = take(money, &amount);
                                assert_eq!(remaining, Eur::create_money(0u64));
                                assert_eq!(taken, Eur::create_amount(15u32));
                        }

                        #[test]
                        fn amount_has_larger_value_than_money_can_represent() {
                                let money = Eur::create_money(255u8);
                                let amount = Eur::create_amount(512u16);
                                let Taken { remaining, taken } = take(money, &amount);
                                assert_eq!(remaining, Eur::create_money(0u8));
                                assert_eq!(taken, Eur::create_amount(255u16));
                        }
                }
        }

        mod pay_with {
                use super::Eur;

                costs![(Price price)];

                mod money_is_same_type {
                        use super::*;
                        use crate::{Change, Currency, PayWith};
                        #[test]
                        fn pay_full_cost() {
                                let money = Eur::create_money(4711u16);
                                let cost = Eur::create_price(4711u16);
                                let Change {
                                        money_back,
                                        left_to_pay,
                                } = cost.pay_with(money);
                                assert_eq!(money_back, Eur::create_money(0u16));
                                assert_eq!(left_to_pay, Eur::create_price(0u16));
                        }

                        #[test]
                        fn pay_partial_cost() {
                                let money = Eur::create_money(12u8);
                                let cost = Eur::create_price(24u8);
                                let Change {
                                        money_back,
                                        left_to_pay,
                                } = cost.pay_with(money);
                                assert_eq!(money_back, Eur::create_money(0u8));
                                assert_eq!(left_to_pay, Eur::create_price(12u8));
                        }

                        #[test]
                        fn pay_more_than_cost() {
                                let money = Eur::create_money(24u8);
                                let cost = Eur::create_price(12u8);
                                let Change {
                                        money_back,
                                        left_to_pay,
                                } = cost.pay_with(money);
                                assert_eq!(money_back, Eur::create_money(12u8));
                                assert_eq!(left_to_pay, Eur::create_price(0u8));
                        }
                }
                mod money_is_a_smaller_type {
                        use super::*;
                        use crate::{Change, Currency, PayWith};
                        #[test]
                        fn pay_full_cost() {
                                let money = Eur::create_money(128u8);
                                let cost = Eur::create_price(128u16);
                                let Change {
                                        money_back,
                                        left_to_pay,
                                } = cost.pay_with(money);

                                assert_eq!(money_back, Eur::create_money(0u8));
                                assert_eq!(left_to_pay, Eur::create_price(0u16));
                        }

                        #[test]
                        fn pay_partial_cost() {
                                let money = Eur::create_money(128u8);
                                let cost = Eur::create_price(255u16);
                                let Change {
                                        money_back,
                                        left_to_pay,
                                } = cost.pay_with(money);
                                assert_eq!(money_back, Eur::create_money(0u8));
                                assert_eq!(left_to_pay, Eur::create_price(127u16));
                        }

                        #[test]
                        fn pay_more_than_cost() {
                                let money = Eur::create_money(255u8);
                                let cost = Eur::create_price(128u16);
                                let Change {
                                        money_back,
                                        left_to_pay,
                                } = cost.pay_with(money);

                                assert_eq!(money_back, Eur::create_money(255u8 - 128));
                                assert_eq!(left_to_pay, Eur::create_price(0u16));
                        }
                }
                mod money_is_a_larger_type {
                        use super::*;
                        use crate::{Change, Currency, PayWith};
                        #[test]
                        fn pay_full_cost() {
                                let money = Eur::create_money(128u16);
                                let cost = Eur::create_price(128u8);
                                let Change {
                                        money_back,
                                        left_to_pay,
                                } = cost.pay_with(money);

                                assert_eq!(money_back, Eur::create_money(0u16));
                                assert_eq!(left_to_pay, Eur::create_price(0u8));
                        }

                        #[test]
                        fn pay_partial_cost() {
                                let money = Eur::create_money(128u16);
                                let cost = Eur::create_price(255u8);
                                let Change {
                                        money_back,
                                        left_to_pay,
                                } = cost.pay_with(money);
                                assert_eq!(money_back, Eur::create_money(0u16));
                                assert_eq!(left_to_pay, Eur::create_price(127u8));
                        }

                        #[test]
                        fn pay_more_than_cost() {
                                let money = Eur::create_money(4096u16);
                                let cost = Eur::create_price(255u8);
                                let Change {
                                        money_back,
                                        left_to_pay,
                                } = cost.pay_with(money);

                                assert_eq!(money_back, Eur::create_money(4096u16 - 255));
                                assert_eq!(left_to_pay, Eur::create_price(0u8));
                        }
                }
        }
}
