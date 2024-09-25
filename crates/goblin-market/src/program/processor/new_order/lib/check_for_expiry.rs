pub struct ExpiryParams {
    // Whether to track block or unix timestamp
    track_block: bool,

    // The last valid block or unix timestamp, depending on the value of
    // track_block. Set value as 0 to disable FOK.
    last_valid_block_or_unix_timestamp_in_seconds: u32,
}

impl ExpiryParams {
    pub fn expired(&self, current_block: u32, current_unix_timestamp_in_seconds: u32) -> bool {
        let ExpiryParams {
            track_block,
            last_valid_block_or_unix_timestamp_in_seconds,
        } = *self;

        last_valid_block_or_unix_timestamp_in_seconds != 0
            && ((track_block && current_block > last_valid_block_or_unix_timestamp_in_seconds)
                || (!track_block
                    && current_unix_timestamp_in_seconds
                        > last_valid_block_or_unix_timestamp_in_seconds))
    }

    #[cfg(test)]
    fn track_expiry(&self) -> bool {
        self.last_valid_block_or_unix_timestamp_in_seconds != 0
    }

    #[cfg(test)]
    fn get_last_valid_block(&self) -> Option<u32> {
        if !self.track_block || self.last_valid_block_or_unix_timestamp_in_seconds == 0 {
            None
        } else {
            Some(self.last_valid_block_or_unix_timestamp_in_seconds)
        }
    }

    #[cfg(test)]
    fn get_last_valid_unix_timestamp(&self) -> Option<u32> {
        if self.track_block || self.last_valid_block_or_unix_timestamp_in_seconds == 0 {
            None
        } else {
            Some(self.last_valid_block_or_unix_timestamp_in_seconds)
        }
    }
}

pub fn order_expired(
    track_block: bool,
    last_valid_block_or_unix_timestamp_in_seconds: u32,
    current_block: u32,
    current_unix_timestamp_in_seconds: u32,
) -> bool {
    last_valid_block_or_unix_timestamp_in_seconds != 0
        && ((track_block && current_block > last_valid_block_or_unix_timestamp_in_seconds)
            || (!track_block
                && current_unix_timestamp_in_seconds
                    > last_valid_block_or_unix_timestamp_in_seconds))
}

pub fn get_last_valid_block(
    track_block: bool,
    last_valid_block_or_unix_timestamp_in_seconds: u32,
) -> Option<u32> {
    if !track_block || last_valid_block_or_unix_timestamp_in_seconds == 0 {
        None
    } else {
        Some(last_valid_block_or_unix_timestamp_in_seconds)
    }
}

pub fn get_last_valid_unix_timestamp(
    track_block: bool,
    last_valid_block_or_unix_timestamp_in_seconds: u32,
) -> Option<u32> {
    if track_block || last_valid_block_or_unix_timestamp_in_seconds == 0 {
        None
    } else {
        Some(last_valid_block_or_unix_timestamp_in_seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_value_for_track_block() {
        let expiry_params = ExpiryParams {
            track_block: true,
            last_valid_block_or_unix_timestamp_in_seconds: 0,
        };

        assert_eq!(expiry_params.track_expiry(), false);
        assert!(expiry_params.get_last_valid_block().is_none());
        assert!(expiry_params.get_last_valid_unix_timestamp().is_none());

        // Current params won't matter if last_valid_block_or_unix_timestamp_in_seconds is zero
        let current_block = 0;
        let current_unix_timestamp_in_seconds = 0;
        assert_eq!(
            expiry_params.expired(current_block, current_unix_timestamp_in_seconds),
            false
        );
    }

    #[test]
    fn test_zero_value_for_track_timestamp() {
        let expiry_params = ExpiryParams {
            track_block: false,
            last_valid_block_or_unix_timestamp_in_seconds: 0,
        };

        assert_eq!(expiry_params.track_expiry(), false);
        assert!(expiry_params.get_last_valid_block().is_none());
        assert!(expiry_params.get_last_valid_unix_timestamp().is_none());

        // Current params won't matter if last_valid_block_or_unix_timestamp_in_seconds is zero
        let current_block = 0;
        let current_unix_timestamp_in_seconds = 0;
        assert_eq!(
            expiry_params.expired(current_block, current_unix_timestamp_in_seconds),
            false
        );
    }

    #[test]
    fn test_for_expired_block() {
        let expiry_params = ExpiryParams {
            track_block: true,
            last_valid_block_or_unix_timestamp_in_seconds: 10,
        };

        assert_eq!(expiry_params.track_expiry(), true);
        assert_eq!(expiry_params.get_last_valid_block().unwrap(), 10);
        assert!(expiry_params.get_last_valid_unix_timestamp().is_none());

        let current_unix_timestamp_in_seconds = 0;

        let current_block_0 = 9;
        assert_eq!(
            expiry_params.expired(current_block_0, current_unix_timestamp_in_seconds),
            false
        );

        // last_valid_block_or_unix_timestamp_in_seconds is inclusive
        let current_block_1 = 10;
        assert_eq!(
            expiry_params.expired(current_block_1, current_unix_timestamp_in_seconds),
            false
        );

        let current_block_2 = 11;
        assert_eq!(
            expiry_params.expired(current_block_2, current_unix_timestamp_in_seconds),
            true
        );
    }

    #[test]
    fn test_for_expired_timestamp() {
        let expiry_params = ExpiryParams {
            track_block: false,
            last_valid_block_or_unix_timestamp_in_seconds: 10,
        };

        assert_eq!(expiry_params.track_expiry(), true);
        assert!(expiry_params.get_last_valid_block().is_none());
        assert_eq!(expiry_params.get_last_valid_unix_timestamp().unwrap(), 10);

        let current_block = 0;

        let current_timestamp_0 = 9;
        assert_eq!(
            expiry_params.expired(current_block, current_timestamp_0),
            false
        );

        // last_valid_block_or_unix_timestamp_in_seconds is inclusive
        let current_timestamp_1 = 10;
        assert_eq!(
            expiry_params.expired(current_block, current_timestamp_1),
            false
        );

        let current_timestamp_2 = 11;
        assert_eq!(
            expiry_params.expired(current_block, current_timestamp_2),
            true
        );
    }
}
