use std::{marker::PhantomData};
use crate::Currency;
use thiserror::Error;
use crate::Cents;

#[derive(Debug, PartialEq, Eq)]
pub struct Price<C: Currency> {
    cents: Cents,
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
        self.cents == other.cents
    }
}

impl <C:Currency> PartialOrd<Money<C>> for Price<C> {
    fn partial_cmp(&self, other: &Money<C>) -> Option<std::cmp::Ordering> {
        self.cents.partial_cmp(&other.cents)
    }
}

impl<C:Currency> Price<C> {
    pub fn new(amount:Cents) -> Self {
        Price{
            cents:amount*100,
            currency:PhantomData,
        }
    }
}

use crate::Money;
impl<C:Currency> Money<C> {
    pub fn pay(mut self:Self, mut price:Price<C>) -> (Self, Price<C>) {
        if price > self {
            let cents = self.cents;
            self.cents = 0;
            price.cents = price.cents - cents;
            (self, price)    
        }
        else {
            let cents = price.cents;
            price.cents = 0;
            self.cents = self.cents - cents;
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
