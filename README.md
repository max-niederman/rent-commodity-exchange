# Rent Commodity Exchange Order Fulfillment

This repository implements a surplus-optimal algorithm for filling orders in a "rent commodity exchange" where agents can buy and sell discrete-time leases on fungible assets.

This was inspired by the [SF Compute](https://sfcompute.com) exchange, where
users can buy and sell GPU resources in hourly increments.

## Implementation

I reduce the problem to a [binary integer linear programming](https://en.wikipedia.org/wiki/Integer_programming) problem. This makes the problem theoretically NP-hard, but in practice the SCIP solver can solve instances with up to thousands of orders in just a few seconds on my laptop.

## License

This code is released under the CC-BY-NC-SA license. See the LICENSE file for details.
