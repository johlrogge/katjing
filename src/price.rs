use std::{marker::PhantomData};
use crate::Currency;
use thiserror::Error;
use crate::MinorUnit;

#[derive(Debug, PartialEq, Eq)]
pub struct Price<C: Currency> {
    minor_unit: MinorUnit,
    currency: PhantomData<C>,
}

#[derive(Error, Debug)]
pub enum PaymentError<C:Currency> {
    #[error("not enough money. Required: {required:?}, Availible {availible:?}")]
    NotEnoughMoney {
        required:Price<C>,
        availible:Money<C>
    }
}

impl <C:Currency> PartialEq<Money<C>> for Price<C> {
    fn eq(&self, other: &Money<C>) -> bool {
        self.minor_unit == other.minor_unit
    }
}

impl <C:Currency> PartialOrd<Money<C>> for Price<C> {
    fn partial_cmp(&self, other: &Money<C>) -> Option<std::cmp::Ordering> {
        self.minor_unit.partial_cmp(&other.minor_unit)
    }
}

impl<C:Currency> Price<C> {
    pub fn new(amount:MinorUnit) -> Self {
        Price{
            minor_unit:amount*100,
            currency:PhantomData,
        }
    }
}

use crate::Money;
impl<C:Currency> Money<C> {
    pub fn pay(mut self:Self, mut price:Price<C>) -> (Self, Price<C>) {
        if price > self {
            let minor_unit = self.minor_unit;
            self.minor_unit = 0;
            price.minor_unit = price.minor_unit - minor_unit;
            (self, price)    
        }
        else {
            let minor_unit = price.minor_unit;
            price.minor_unit = 0;
            self.minor_unit = self.minor_unit - minor_unit;
            (self, price)
        }
    }
    
    pub fn try_pay(self, price:Price<C>) -> Result<Self, PaymentError<C>> {
        if price <= self {
            let (change, _) = self.pay(price);
            Ok(change)
        }
        else {
            Err(PaymentError::NotEnoughMoney { required: price, availible: self })    
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Money;
    use crate::test::SEK;
    use super::{Price, PaymentError};
    #[test]
    fn pay_full_price_with_exact_amount() {
        let sek_200 = Money::<SEK>::new(200);
        let price_sek_200 = Price::<SEK>::new(200);
        let (remaining_money, remaining_price) = sek_200.pay(price_sek_200);
        assert_eq!(remaining_price, Price::new(0));
        assert_eq!(remaining_money, Money::new(0));
    }
    
    #[test]
    fn pay_price_that_is_lower_than_money() {
        let sek_200 = Money::<SEK>::new(200);
        let price_sek_190= Price::<SEK>::new(190);
        let (remaining_money, remaining_price) = sek_200.pay(price_sek_190);
        assert_eq!(remaining_price, Price::new(0));
        assert_eq!(remaining_money, Money::new(10));
    }
    
    #[test]
    fn pay_price_that_is_higher_than_money() {
        let sek_190= Money::<SEK>::new(190);
        let price_sek_200= Price::<SEK>::new(200);
        let (remaining_money, remaining_price) = sek_190.pay(price_sek_200);
        assert_eq!(remaining_price, Price::new(10));
        assert_eq!(remaining_money, Money::new(0));
    }
    
    #[test]
    fn try_pay_with_exact_amount() -> Result<(), PaymentError<SEK>> {
        let sek_200 = Money::<SEK>::new(200);
        let price_sek_200 = Price::<SEK>::new(200);
        let remaining_money = sek_200.try_pay(price_sek_200)?;
        assert_eq!(Money::<SEK>::new(0), remaining_money);
        Ok(())
    }
}
