use crate::{AggregationRule, ModularCandle, Trade};

/// The classic time based aggregation rule,
/// creating a new candle every n milliseconds
pub struct TimeRule {
    /// If true, the reference timestamp needs to be reset
    init: bool,

    // The timestamp this rule uses as a reference
    reference_timestamp: i64,

    // The period for the candle in seconds
    // constants can be used nicely here from constants.rs
    // e.g.: M1 -> 1 minute candles
    period_s: i64,
}

impl TimeRule {
    /// Create a new instance of the time rule,
    /// with a given candle period in seconds
    pub fn new(period_s: i64) -> Self {
        Self {
            init: true,
            reference_timestamp: 0,
            period_s,
        }
    }
}

impl<C> AggregationRule<C> for TimeRule
where
    C: ModularCandle,
{
    fn should_trigger(&mut self, trade: &Trade, _candle: &C) -> bool {
        if self.init {
            self.reference_timestamp = trade.timestamp;
            self.init = false;
        }
        let should_trigger = trade.timestamp - self.reference_timestamp > self.period_s * 1000;
        if should_trigger {
            self.init = true;
        }

        should_trigger
    }
}