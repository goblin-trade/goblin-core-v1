use crate::types::LegMarker;

#[derive(Default, PartialEq)]
pub struct MatchResult<In: LegMarker> {
    /// Lots of input token traded in on match
    pub free_matching_lots_in: In::MatchingLots,

    /// Lots of output token obtained on match
    pub locked_matching_lots_out: <In::Opposite as LegMarker>::MatchingLots,

    /// Lots of output token released on self trade
    pub released_by_self_trade: <In::Opposite as LegMarker>::MatchingLots,
}

#[cfg(test)]
mod tests {
    use crate::types::Base;

    use super::*;

    #[test]
    fn test_default_equal() {
        let result_0 = MatchResult::<Base>::default();
        let result_1 = MatchResult::<Base>::default();

        if result_0 == result_1 {}
        // if result_0.eq(result_1) {}
    }
}
