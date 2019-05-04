# A tour of Katjing

Strongly typed money.

Katjing is a money library that attempts to check as much as possible at compile time. If possible with zero run-time overhead.
If a compromise has to be made, chose the safer option. Katjing is mostly concerned with structural correctness and outsources
the details to other types. That is why [Money] is generic. Lastly  Katjing tries not to do surprising things!

For example:

 * [Money] represents currency as phantom data allowing compile time checking of not mixing currencies while not using more memory than required to store the actual [Amount].
 * [Money] is unsigned, can never go below zero, does not allow fractions and will return an error on overflows.
 * [Money] is a representation of actual money in hand, when paying a [Cost] the money is consumed and returns the remaining [Cost] as change. This way, the same money cannot be used to pay more than one [Amount].
 * All conversions between currencies have to be explicitly specified when needed.
 * Katjing is experimental and has not been reviewed for production use, use at your own risk*

 Some code will probably illustrate the above much better.

 ## Currencies
 Katjing treats currencies as separate types. That means you cannot mix amounts of different currencies.
 Let's define some currencies and create some money:

```rust
# #[macro_use] extern crate katjing;
# fn main () {
use katjing::prelude::*;
currencies![(IDR Main), (EUR Cent), (KWD Mill)];
let some_idr = IDR::create_money(18u8);
let some_eur = EUR::create_money(40_000u128);
let some_kwd = KWD::create_money(64_000u32);
# }
```
 As you can see you can create money of different types, and you create them from a currency.
 The currency is just phantomdata and is only relevant during compile time and allows rust to
 make sure that you don't do nonsensical things by mistake [^adding]

 When you specify a currency with the `currencies!` macro you also specify the minimal representable unit of the currency.
 The minimal representable unit can be
 <dl>
 <dt>Main</dt><dd>Has no subunit as is the case with the Indonesian Rupiah <em>IDR</em> in the example</dd>
 <dt>Cent</dt><dd>Subunit is 1/100 of the main unit. This is the most common subunit type used</dd>
 <dt>Mill</dt><dd>Subunit is 1/1000 of the main unit as is the case with the Kuwaiti Dinar <em>KWD</em> used in the example</dd>
 </dl>

 The subunit is abstract and is not concerned with the actual names of the subunits. For instance: SEK would declare
 `Cent` even though the actual name of the subunit is *öre*.

## Units
 In the above examples we have created amounts from the main unit. But what if we want to use those cents and mills?

```rust
# #[macro_use] extern crate katjing;
# fn main () {
use katjing::prelude::*;
currencies![(idr Main), (eur Cent), (kwd Mill)];
let _some_idr = idr::Main::create_money(18u8);
let _some_eur = eur::Cent::create_money(40_032u128);
let _some_kwd = kwd::Mill::create_money(64_186u32);
# }
```
Of course you can only create the subunits that are declared
```rust,ignore
# #[macro_use] extern crate katjing;
# fn main () {
# use katjing::prelude::*;
# currencies![(IDR Main), (EUR Cent), (KWD Mill)];
let some_idr = IDR::Cent::create_money(18u8);
# }
```
*You cannot create cents for a currency with only main unit.*

```rust,ignore
# #[macro_use] extern crate katjing;
# fn main () {
# use katjing::prelude::*;
# currencies![(IDR Main), (EUR Cent), (KWD Mill)];
let some_eur = EUR::Mill::create_money(18u8);
# }
```
*You cannot create mills for a currency with cents.*

```rust,ignore
# #[macro_use] extern crate katjing;
# fn main () {
# use katjing::prelude::*;
# currencies![(IDR Main), (EUR Cent), (KWD Mill)];
let some_kwd = KWD::Cent::create_money(18u8);
# }
```
*You cannot create cents for a currency with mills.*

But what good is wealth if you can't spend it on anything?

[^adding]: like adding 2 SEK to 10 USD, Then you would just have 2 SEK and 10 USD unless you BUY
           USD for your SEK which is a different operation entirely)

## Costs
Just like currencies katjing allows you to define costs. A cost is something that can be covered (payed), like
a Price, Shipping, VAT, or Interest. We will get back to how to calculate VAT and interests in a bit.

```rust
# #[macro_use] extern crate katjing;
# currencies![(idr Main), (eur Cent), (kwd Mill)];
costs![(Shipping shipping), (Price price)];
fn main () {
  let _shipping_eur = eur::Main::create_shipping(1u8);
  let _shipping_idr = idr::Main::create_shipping(2u8);
  let _price_eur = eur::Main::create_price(100u16);
  let _price_idr = idr::Main::create_price(128u16);
}
```
As you may notice you can choose *storage type* for your costs and your money. You may not expect shipping in € to be a very large amount
so you may choose a small type to represent shipping while you would choose something bigger to represent national debt.
You may also notice that all values are unsigned. This is also by design. Negative money does not exist, that is only for calculation. Katjing uses types instead of signs as we will see.
Lastly, costs and money are created from a currency. **You cannot mix currencies**
From now on we will assume the above costs and currencies are defined.

As mentioned initially Katjing tries to prevent as many errors as possible at compile time. Here are a few examples we can demonstrate with what we know:

```rust,ignore
# #[macro_use] extern crate katjing;
# use katjing::prelude::*;
# currencies![(IDR Main), (EUR Cent), (KWD Mill)];
# costs![(Shipping shipping), (Price price)];
# fn main () {
    let shipping_eur:Shipping<u8, IDR::Main> = EUR::Main::create_shipping(1u8);
# }
```
*You cannot assign a cost to a cost different currency.*

```rust,ignore
# #[macro_use] extern crate katjing;
# use katjing::prelude::*;
# currencies![(IDR Main), (EUR Cent), (KWD Mill)];
# costs![(Shipping shipping), (Price price)];
# fn main () {
    let shipping_eur:Shipping<u8, EUR::Main> = EUR::Main::create_price(1u8);
# }
```
*You cannot assign a cost to a cost of a different type.*

```rust,ignore
# #[macro_use] extern crate katjing;
# use katjing::prelude::*;
# currencies![(IDR Main), (EUR Cent), (KWD Mill)];
# fn main () {
    let money_usd:Money<u8, IDR::Main> = EUR::create_money(1u8);
# }
```
*You cannot assign money to money of a different currency.*

```rust
# #[macro_use] extern crate katjing;
# use katjing::prelude::*;
# currencies![(IDR Main), (EUR Cent), (KWD Mill)];
# fn main () {
    EUR::create_money(1u8);
# }
```
*Warns if you don't assign created money to something.*

## Paying costs

While creating costs and money can be fun. Let's use it for something:

```rust
# #[macro_use] extern crate katjing;
use katjing::prelude::*;
// define costs and currencies
# costs![(Price price), (Shipping shipping)];
# currencies![(IDR Main), (EUR Cent), (KWD Mill)];
# fn main() {
let shipping = EUR::Main::create_shipping(12u8);
let money = EUR::Main::create_money(1_000u16);

let Change{money_back, left_to_pay} = shipping.pay_with(money);
assert_eq!(money_back, EUR::Main::create_money(1_000u16-12));
assert_eq!(left_to_pay, EUR::Main::create_shipping(0u8));
# }
```

There are two important things to note here:

1. `pay_with` returns `Change` containing any remaining money and cost after the payment
2. `pay_with` consumes `money`, it cannot be used again.

Let's try to use money after the payment:


```rust,ignore
# #[macro_use] extern crate katjing;
# use katjing::prelude::*;
# costs![(Price price), (Shipping shipping)];
# currencies![(IDR Main), (EUR Cent), (KWD Mill)];
# fn main() {
let shipping = EUR::Main::create_shipping(12u8);
let price = EUR::Main::create_price(1_000u16);
let money = EUR::Main::create_money(1_014u16);

let Change{ money_back, left_to_pay } = price.pay_with(money);
let Change{ money_back, left_to_pay } = shipping.pay_with(money); // <- fails: money has already been used to pay price with
# }
```
*`money` has moved when paying `price`, it cannot be used again. This prevents paying the cost with money we don't have.*

Let's fix the code above:

```rust
# #[macro_use] extern crate katjing;
# use katjing::prelude::*;
# costs![(Price price), (Shipping shipping)];
# currencies![(IDR Main), (EUR Cent), (KWD Mill)];
# fn main() {
let shipping = EUR::Main::create_shipping(12u8);
let price = EUR::Main::create_price(1_000u16);
let money = EUR::Main::create_money(1_014u16);

// Note below how we destructure `money_back` to `money`.
// This makes our second payment use the `money_back` from the first payment.
// That way we are not trying to use the moved initially created `money` twice
let Change{ money_back: money, left_to_pay: price } = price.pay_with(money);
let Change{ money_back: money, left_to_pay: shipping } = shipping.pay_with(money);

assert_eq!( money, EUR::create_money(2u16));
assert_eq!( price, EUR::Main::create_price(0u16));
assert_eq!( shipping, EUR::Main::create_shipping(0u8));
# }
```

[Money]: struct.Money.html
[Amount]: struct.Amount.html
[Cost]: trait.Cost.html
