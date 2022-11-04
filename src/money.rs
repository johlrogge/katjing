use std::{fmt::Display, marker::PhantomData};
use crate::currency::Currency;
use crate::MinorUnit;

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Money<C: Currency> {
    pub(crate) minor_unit: MinorUnit,
    currency: PhantomData<C>,
}

impl<C: Currency> Money<C> {
    /// Create new Money instances with new. You need to specify
    /// a currency. New assumes Money is created in it's main unit (like in dollar, not cent)
    /// ```
    /// # use katjing::Money;
    /// # use katjing::test::{SEK, EUR};
    /// let one_euro = Money::<EUR>::new(1);
    /// let ten_sek = Money::<SEK>::new(10);
    /// ```
    ///
    /// Since money is generic over Currency you cannot accidentally
    /// assign EUR to SEK for instance.
    ///
    /// ```compile_fail
    /// # use katjing::Money;
    /// # use katjing::test::{EUR, SEK};
    /// let one_euro = Money::<EUR>::new(1);
    /// let euro:Money<SEK> = one_euro;
    /// ```
    pub fn new(value: MinorUnit) -> Money<C> {
        Self::in_minor_unit(value * 100)
    }
    /// If you want to create money with it's fractional representation
    /// you use *in_minor_unit*
    /// ```
    /// # use katjing::Money;
    /// # use katjing::test::EUR;
    /// let one_euro = Money::<EUR>::new(1);
    /// let another_one_euro = Money::<EUR>::in_minor_unit(100);
    /// assert_eq!(one_euro, another_one_euro);
    /// ```
    pub fn in_minor_unit(minor_unit: MinorUnit) -> Money<C> {
        Money {
            minor_unit,
            currency: PhantomData,
        }
    }
}

impl<C: Currency> Display for Money<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{:02} {}",
            self.minor_unit / (C::MINOR_UNIT as u128),
            self.minor_unit % (C::MINOR_UNIT as u128),
            C::ALPHABETIC_CODE
        )
    }
}

#[cfg(test)]
mod display {
    use crate::test::SEK;
    use crate::Money;
    #[test]
    fn shows_value() {
        let one_sek = Money::<SEK>::new(1);
        assert_eq!(format!("{}", one_sek), "1.00 SEK")
    }

    #[test]
    fn shows_minor_unit() {
        let one_thirtythree_sek = Money::<SEK>::in_minor_unit(133);
        assert_eq!(format!("{}", one_thirtythree_sek), "1.33 SEK");
    }
}

#[cfg(test)]
mod compare {
    use crate::{Money, test::SEK};

    #[test]
    fn one_sek_eq_one_sek() {
        let one_sek = Money::<SEK>::new(1);
        let another_one_sek = Money::<SEK>::new(1);
        assert_eq!(one_sek, another_one_sek);
    }

    #[test]
    fn one_sek_ne_two_sek() {
        let one_sek = Money::<SEK>::new(1);
        let two_sek = Money::<SEK>::new(2);
        assert_ne!(one_sek, two_sek);
    }

    #[test]
    fn one_thirty_gt_one_thirtyone() {
        let sek_1_30 = Money::<SEK>::in_minor_unit(130);
        let sek_1_31 = Money::<SEK>::in_minor_unit(131);
        assert!(sek_1_31 > sek_1_30);
    }
}
