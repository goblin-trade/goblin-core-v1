use crate::state::{ArbContext, ContextActions};

/// Checks whether an orders have expired. It optimizes for gas by lazy loading
/// block number and block timestamp only when they are needed.
///
/// Eager loading both costs 4 gas. The gas saved is small but percentage improvement
/// should be good.
pub struct ExpiryChecker {
    block_number: Option<u32>,
    block_timestamp: Option<u32>,
}

impl ExpiryChecker {
    /// Creates a new `ExpiryChecker` with no cached values.
    pub fn new() -> Self {
        ExpiryChecker {
            block_number: None,
            block_timestamp: None,
        }
    }

    /// Returns the block number. If it's not already cached, it is fetched from the context
    /// and stored for future use.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context from which the block number will be retrieved if not cached.
    fn block_number(&mut self, ctx: &ArbContext) -> u32 {
        self.block_number.unwrap_or_else(|| {
            let block_number = ctx.block_number() as u32;
            self.block_number = Some(block_number);

            block_number
        })
    }

    /// Returns the block timestamp. If it's not already cached, it is fetched from the context
    /// and stored for future use.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context from which the block timestamp will be retrieved if not cached.
    fn block_timestamp(&mut self, ctx: &ArbContext) -> u32 {
        self.block_timestamp.unwrap_or_else(|| {
            let block_timestamp = ctx.block_timestamp() as u32;
            self.block_timestamp = Some(block_timestamp);

            block_timestamp
        })
    }

    /// Checks whether an order is expired based on the current block number or timestamp.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context used to fetch block number or timestamp if they are not cached.
    /// * `track_block` - A boolean indicating whether the expiration check is based on block number (`true`) or timestamp (`false`).
    /// * `last_valid_block_or_unix_timestamp_in_seconds` - The last valid block number or timestamp before expiration.
    ///
    /// # Returns
    ///
    /// * `true` if the order has expired, otherwise `false`.
    pub fn is_expired(
        &mut self,
        ctx: &ArbContext,
        track_block: bool,
        last_valid_block_or_unix_timestamp_in_seconds: u32,
    ) -> bool {
        last_valid_block_or_unix_timestamp_in_seconds != 0
            && ((track_block
                && self.block_number(ctx) > last_valid_block_or_unix_timestamp_in_seconds)
                || (!track_block
                    && self.block_timestamp(ctx) > last_valid_block_or_unix_timestamp_in_seconds))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod expiry_checker {
        use super::{ArbContext, ExpiryChecker};

        #[test]
        fn test_lazy_load() {
            let block_number = 100;
            let block_timestamp = 500;
            let ctx = ArbContext::new_with_block_details(block_number, block_timestamp);
            let mut expiry_checker = ExpiryChecker::new();

            let read_block_number = expiry_checker.block_number(&ctx);
            assert_eq!(read_block_number, block_number as u32);
            assert_eq!(expiry_checker.block_number.unwrap(), block_number as u32);

            let read_block_timestamp = expiry_checker.block_timestamp(&ctx);
            assert_eq!(read_block_timestamp, block_timestamp as u32);
            assert_eq!(
                expiry_checker.block_timestamp.unwrap(),
                block_timestamp as u32
            );
        }

        #[test]
        fn test_zero_value_for_track_block() {
            let block_number = 100;
            let block_timestamp = 500;
            let ctx = ArbContext::new_with_block_details(block_number, block_timestamp);

            let mut expiry_checker = ExpiryChecker::new();

            let track_block = true;
            let last_valid_block_or_unix_timestamp_in_seconds = 0;
            let expired = expiry_checker.is_expired(
                &ctx,
                track_block,
                last_valid_block_or_unix_timestamp_in_seconds,
            );

            assert_eq!(expired, false);
            assert!(expiry_checker.block_number.is_none());
            assert!(expiry_checker.block_timestamp.is_none());
        }

        #[test]
        fn test_zero_value_for_track_timestamp() {
            let block_number = 100;
            let block_timestamp = 500;
            let ctx = ArbContext::new_with_block_details(block_number, block_timestamp);

            let mut expiry_checker = ExpiryChecker::new();

            let track_block = false;
            let last_valid_block_or_unix_timestamp_in_seconds = 0;
            let expired = expiry_checker.is_expired(
                &ctx,
                track_block,
                last_valid_block_or_unix_timestamp_in_seconds,
            );

            assert_eq!(expired, false);
            assert!(expiry_checker.block_number.is_none());
            assert!(expiry_checker.block_timestamp.is_none());
        }

        #[test]
        fn test_block_number() {
            let block_number = 100;
            let block_timestamp = 500;
            let ctx = ArbContext::new_with_block_details(block_number, block_timestamp);

            let mut expiry_checker = ExpiryChecker::new();

            let track_block = true;

            let last_valid_block_0 = 101;
            let expired_0 = expiry_checker.is_expired(&ctx, track_block, last_valid_block_0);
            assert_eq!(expired_0, false);
            assert_eq!(expiry_checker.block_number.unwrap(), block_number as u32);
            assert!(expiry_checker.block_timestamp.is_none());

            let last_valid_block_1 = 100;
            let expired_1 = expiry_checker.is_expired(&ctx, track_block, last_valid_block_1);
            assert_eq!(expired_1, false);

            let last_valid_block_2 = 99;
            let expired_2 = expiry_checker.is_expired(&ctx, track_block, last_valid_block_2);
            assert_eq!(expired_2, true);
        }

        #[test]
        fn test_block_time() {
            let block_number = 100;
            let block_timestamp = 500;
            let ctx = ArbContext::new_with_block_details(block_number, block_timestamp);

            let mut expiry_checker = ExpiryChecker::new();

            let track_block = false;

            let last_valid_timestamp_0 = 501;
            let expired_0 = expiry_checker.is_expired(&ctx, track_block, last_valid_timestamp_0);
            assert_eq!(expired_0, false);
            assert!(expiry_checker.block_number.is_none());
            assert_eq!(
                expiry_checker.block_timestamp.unwrap(),
                block_timestamp as u32
            );

            let last_valid_timestamp_1 = 500;
            let expired_1 = expiry_checker.is_expired(&ctx, track_block, last_valid_timestamp_1);
            assert_eq!(expired_1, false);

            let last_valid_timestamp_2 = 499;
            let expired_2 = expiry_checker.is_expired(&ctx, track_block, last_valid_timestamp_2);
            assert_eq!(expired_2, true);
        }
    }
}
