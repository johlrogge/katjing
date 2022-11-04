# Katjing

A money library exploring the aplicability of Rusts unique language features
for safe money management.

Katjing aims to make money management as safe as possible by checking as much
as possible with static typing including rusts borrow checking and life time
management.

## Feature overview

*   [x] **Clear ownership** Rust tracks ownership
    * you can't give the same money instance to multiple parties
    * you can't spend money while another instance is borrowing it
*   [x] **Generic over currency** You can't mix money with different currencies
*   [x] **Cent representation** No floating point in money representation, any commas or dots in money representation is for  display purposes only, not an attribute of Money
*   [ ] **Separate money and amounts** Amounts are not Money 
*   [x] **Separately modelled prices** Prices can be paid, consume money and leave change.

## Contents

*   [What is katjing?](#what-is-this)
*   [When should I use katjing?](#when-should-i-use-this)
*   [Usage](#usage)

## What is this?

Katjing is a Money library meant to be used by applications and libraries that
handle monetary transactions and/or calculations.

## Why should I use this?

For now if you are feeling adventureous.

### Usage

Money can be crated in main parts or cents

```rust
use {
  katjing::{
    Money,
    test::SEK
  }
};
let one_sek=Money::<SEK>::new(1);
let one_sek_again=Money::<SEK>::in_cents(100);
assert_eq!(one_sek, one_sek_again);
```

Since money is generic over currencies they can't be mixed (since they are different types)
```rust compile_fail
use {
  katjing::{
    Money,
    test::{SEK, EUR},
  }
};
let one_sek=Money::<SEK>::new(1);
let one_eur=Money::<EUR>::new(1);
assert_eq!(one_sek, one_eur); // <-- will not compile
```
The above example will yeild an error that looks something like this:
```output
error[E0308]: mismatched types
  --> src/lib.rs:96:1
   |
12 | assert_eq!(one_sek, one_eur);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `SEK`, found struct `EUR`
   |
   = note: expected struct `Money<SEK>`
              found struct `Money<EUR>`
```
Let's make things more interesting and introduce rust's borrowchecking.

```rust compile_fail
use {
  katjing::{
    Money,
    test::SEK
  }
};

fn moving_fn(money:Money<SEK>) {
  // do something with owned money
}

let one_sek=Money::<SEK>::new(1);
moving_fn(one_sek);
moving_fn(one_sek); // <-- does not compile
```

We crate one sek, give it away and then try to give it away again. Rust will
complain that you are giving away money that are no longer yours like this:

```output
error[E0382]: use of moved value: `one_sek`
  --> src/lib.rs:125:11
   |
15 | let one_sek=Money::<SEK>::new(1);
   |     ------- move occurs because `one_sek` has type `Money<SEK>`, which does not implement the `Copy` trait
16 | moving_fn(one_sek);
   |           ------- value moved here
17 | moving_fn(one_sek);
   |           ^^^^^^^ value used here after move
```

Similarily you cannot lend something you have given away to someone else
```rust compile_fail
use {
  katjing::{
    Money,
    test::SEK
  }
};

fn moving_fn(money:Money<SEK>) {
  // do something with owned money
}

fn borrowing_fn(money:&Money<SEK>) {
  // do something with borrowed money  
}

let one_sek=Money::<SEK>::new(1);
moving_fn(one_sek);
borrowing_fn(&one_sek); // <-- does not compile
```

Yields the following error:
``` output
error[E0382]: borrow of moved value: `one_sek`
  --> src/lib.rs:162:14
   |
19 | let one_sek=Money::<SEK>::new(1);
   |     ------- move occurs because `one_sek` has type `Money<SEK>`, which does not implement the `Copy` trait
20 | moving_fn(one_sek);
   |           ------- value moved here
21 | borrowing_fn(&one_sek); // <-- does not compile
   |              ^^^^^^^^ value borrowed here after move
```
While this is basic Rust it is pretty unique and I beleive this can be very useful for managing money,
also, money can be equally useful for learning about Rust's borrow checker.

### Prices

Prices are not money, they state a cost and can be paid for goods to change owners. Katjing models
prices as consumers of money.

```rust
use {
  katjing::{
    Money,
    Price,
    test::SEK
  }
};

let ten_sek=Money::<SEK>::new(10);
let nine_sek_price=Price::<SEK>::new(9);
let (change, left_to_pay) = ten_sek.pay(nine_sek_price);
assert_eq!(left_to_pay, Price::<SEK>::new(0));
assert_eq!(change, Money::<SEK>::new(1));
```
