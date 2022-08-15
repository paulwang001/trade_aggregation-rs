use chrono::naive::NaiveDateTime;

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
/// Defines a taker trade
pub struct Trade {
    /// Timestamp, assumed to be in milliseconds
    pub timestamp: i64,

    /// Price of the asset
    pub price: f64,

    /// Size of the trade
    /// negative values indicate a taker Sell order
    pub size: f64,
}

/*
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[deprecated]
/// Defines a Candle
pub struct Candle {
    /// latest timestamp of last received trade
    pub timestamp: i64,
    /// open price of candle
    pub open: f64,
    /// high price of candle
    pub high: f64,
    /// low price of candle
    pub low: f64,
    /// close price of candle
    pub close: f64,
    /// summed taker volume of all trades in candle
    pub volume: f64,
    /// #buys / #trades
    pub directional_trade_ratio: f64,
    /// buy_volume / volume
    pub directional_volume_ratio: f64, // buy_volume / volume // in range [0, 1]
    /// number of taker trades observed in candle
    pub num_trades: i32,
    /// arithmetic mean of price
    pub arithmetic_mean_price: f64,
    /// volume weighted price
    pub weighted_price: f64,
    /// standard deviation of trade prices
    pub std_dev_prices: f64,
    /// standard deviation of trade sizes
    pub std_dev_sizes: f64,
    /// measure of candle creation time: 1.0 / time_in_seconds
    pub time_velocity: f64,
}

impl std::fmt::Display for Candle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(ts: {:?}, o: {:.8}, h: {:.8}, l: {:.8}, c: {:.8}, wp: {:.8}, v: {:.2}, dtr: {:.4}, dvr: {:.4}, #t: {}, σ_price: {:.4}, σ_size: {:.4}, tv: {:.4})",
               NaiveDateTime::from_timestamp(self.timestamp / 1000, (self.timestamp % 1000) as u32),
               self.open,
               self.high,
               self.low,
               self.close,
               self.weighted_price,
               self.volume,
               self.directional_trade_ratio,
               self.directional_volume_ratio,
               self.num_trades,
               self.std_dev_prices,
               self.std_dev_sizes,
               self.time_velocity,
        )
    }
}
*/

/// Defines how to aggregate trade size
/// either by Base currency or Quote Currency
/// assumes trades sizes are denoted in Quote
/// e.g.: buy 10 contracts of BTC would be trade size of 10
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum By {
    /// when aggregating by Base, divide size by price for volume sum
    Base,
    /// when aggregating by Quote, take the raw trade size for volume sum
    /// as the assumption is that Trade size is denoted in Quote
    Quote,
}