# Pseudo tree

The pseudo tree consists of following data structures

- TickGroupList: Array of active TickGroup indices stored continuously as u16 integers in U256 slots. It is sorted in ascending order for asks and descending order for bids.

- TickGroup: A TickGroup slot consists of 32 TickHeaders. This allows a max tick size of 2^16 * 32 = 2^21.

- TickHeader: gives the number of active orders on a tick and head index

- PseudoTreeRestingOrder: Resting orders have a key (price, index). Resting orders on a tick
stored in a virtual circular array with a max limit of 16 orders per tick.

## Operations

1. Placing post-only order
  - Convert the order's tick price to TickGroupIndex and add it to the TickGroupList if it doesn't exist already. Shift the ticks
  on the right side (greater than) of the order's price
  - Increment the order count on the tick header. Ensure that the max size is not exceeded
  - Insert the resting order

2. Taker order
  - Read first slot from the ticks array. Decode the ticks and use them to fetch the resting
  - orders. Perform operations on the resting orders.
  - Depleted ticks must be removed from the ticks array


Tick operations needed

1. Iterate tick slots one by one for taker orders
  - Save fetched slots into a vector. The ticks vector will be used when updating the slots

2. Find whether tick exists*: We can directly look at the tick header.

3. Read all ticks: Needed for new tick insertions

4. Write all slots to the right
