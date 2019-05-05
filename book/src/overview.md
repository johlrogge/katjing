# Overview

Strongly typed money.

Katjing is a money library that attempts to check as much as possible at compile time. If possible with zero run-time overhead.
If a compromise has to be made, chose the safer option. Katjing is mostly concerned with structural correctness and outsources
the details to other types. That is why `Money` is generic. Lastly  Katjing tries not to do surprising things!

For example:

 * `Money` represents currency as phantom data allowing compile time checking of not mixing currencies while not using more memory than required to store the actual [Amount].
 * `Money` is unsigned, can never go below zero, does not allow fractions and will return an error on overflows.
 * `Money` is a representation of actual money in hand, when paying a `Cost` the money is consumed and returns the remaining `Cost` as change. This way, the same money cannot be used to pay more than one [Amount].
 * All conversions between currencies have to be explicitly specified when needed.
 * Katjing is experimental and has not been reviewed for production use, use at your own risk*
