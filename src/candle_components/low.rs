use crate::{CandleComponent, Trade};

/// This 'CandleComponent' keeps track of the low price
#[derive(Debug, Clone)]
pub struct Low {
    low: f64,
}

impl Default for Low {
    fn default() -> Self {
        // ensure the initial value is maximal,
        // so any subsequent calls to 'update' will set the proper low value
        Self { low: f64::MAX }
    }
}

impl CandleComponent for Low {
    fn value(&self) -> f64 {
        self.low
    }

    fn update(&mut self, trade: &Trade) {
        if trade.price < self.low {
            self.low = trade.price;
        }
    }

    fn reset(&mut self) {
        self.low = f64::MAX;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn low() {
        let mut m = Low::default();
        for t in &crate::candle_components::tests::TRADES {
            m.update(t);
        }
        assert_eq!(m.value(), 100.0);
    }
}