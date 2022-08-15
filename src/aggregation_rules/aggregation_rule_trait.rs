use crate::Trade;

/// Defines under what conditions one aggregation period is finished
pub trait AggregationRule<C> {
    /// The main method defining when the aggregation is done
    ///
    /// # Arguments:
    /// trade: The most recent taker trade (tick) information
    /// candle: Some generic Candle, allowing for information driven decision making
    ///
    /// # Returns:
    /// if true, the aggregation period is finished and a Candle can be emitted
    /// else the aggregation needs to continue
    fn should_trigger(&mut self, trade: &Trade, candle: &C) -> bool;
}
