use crate::{AggregationRule, ModularCandle, TakerTrade};

/// The classic time based aggregation rule,
/// creating a new candle every n seconds
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

impl<C, T> AggregationRule<C, T> for TimeRule
where
    C: ModularCandle<T>,
    T: TakerTrade,
{
    fn should_trigger(&mut self, trade: &T, _candle: &C) -> bool {
        if self.init {
            self.reference_timestamp = trade.timestamp();
            self.init = false;
        }
        let should_trigger = trade.timestamp() - self.reference_timestamp > self.period_s * 1000;
        if should_trigger {
            self.init = true;
        }

        should_trigger
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        aggregate_all_trades, load_trades_from_csv,
        plot::{plot_ohlc_candles, OhlcCandle},
        GenericAggregator, Trade, M15,
    };

    use super::*;

    #[test]
    fn time_candles_plot() {
        let trades = load_trades_from_csv("data/Bitmex_XBTUSD_1M.csv").unwrap();

        let mut aggregator =
            GenericAggregator::<OhlcCandle, TimeRule, Trade>::new(TimeRule::new(M15));
        let candles = aggregate_all_trades(&trades, &mut aggregator);
        println!("got {} candles", candles.len());

        plot_ohlc_candles(&candles, "img/time_candles_plot.png", (2560, 1440)).unwrap();
    }
}
