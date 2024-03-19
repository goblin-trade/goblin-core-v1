# Pseudo tree

The iterable tick map consists of following data structures

- OrderAtTick: An 8 bit bitmap representing active orders at a tick

- Bitmap: A Bitmap consists of 32 OrderAtTicks. This allows a max tick size of 2^16 * 32 = 2^21.

- BitmapList: Array of active Bitmap indices stored continuously as u16 integers in U256 slots. It is sorted in ascending order for asks and descending order for bids. It tells which bitmaps are active, allowing us to iterate.

- SlotRestingOrder: Resting orders have a key (price, index). Upto 8 orders can be stored at a tick.

## Operations

1. Placing post-only order
  - Calculate bitmap index for the tick. Find the corresponding `OrderAtTick` and turn on the bit if it was off.
  - If the bitmap state was empty before turning on the bit, add the bitmap index to `BitmapList`. The index should be added on the right side. The bitmaps are arranged in ascending orders for bids and descending order for asks.
  on the right side (greater than) of the order's price
  - Increment the order count on the tick header. Ensure that the max size is not exceeded
  - Insert the resting order

2. Taker order
  - Fetch bitmap indices by looping from the end of bitmap_list
  - Decode the bitmap
  - Read resting order slots in ascending order of index
  - Depleted ticks must be removed from the ticks array

Tick operations needed

1. Iterate tick slots one by one for taker orders
  - Save fetched slots into a vector. The ticks vector will be used when updating the slots

2. Find whether tick exists*: We can directly look at the tick header.

3. Read all ticks: Needed for new tick insertions

