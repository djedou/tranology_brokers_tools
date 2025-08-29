use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cmp::Ordering;
use eq_float::F64;


#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum CandleColor {
    #[default]
    None,
    Green,
    Red
}



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Candle {
    pub open_time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub close_time: i64,
    pub volume: f64,
    pub quote_asset_volume: f64,
    pub upper_wick: f64,
    pub lower_wick: f64,
    pub real_body: f64,
    pub color: CandleColor
}

impl Candle {
    pub fn is_green(&self) -> bool {
        match self.color {
            CandleColor::Green => true,
            _ => false
        }
    }

    pub fn is_red(&self) -> bool {
        match self.color {
            CandleColor::Red => true,
            _ => false
        }
    }
}


impl From<&[Value; 8]> for Candle {
    fn from(values: &[Value; 8]) -> Self {
        let open = values[1].as_str().map_or(0.0, |v| v.parse::<f64>().map_or(0.0, |d| d));
        let high = values[2].as_str().map_or(0.0, |v| v.parse::<f64>().map_or(0.0, |d| d));
        let low = values[3].as_str().map_or(0.0, |v| v.parse::<f64>().map_or(0.0, |d| d));
        let close = values[4].as_str().map_or(0.0, |v| v.parse::<f64>().map_or(0.0, |d| d));
        let body = (open - close).abs();

        let (color, upper_wick, lower_wick) = match F64(open).cmp(&F64(close)) {
            Ordering::Less => {
                (CandleColor::Green, high - close, open - low)
            },
            Ordering::Equal => (CandleColor::None, high - close, open - low),
            Ordering::Greater => {
                (CandleColor::Red, high - open, close - low)
            }
        };

        Candle {
            open_time: values[0].as_i64().map_or(0, |v| v),
            open,
            high: high.clone(),
            low: low.clone(),
            close,
            volume: values[5].as_str().map_or(0.0, |v| v.parse::<f64>().map_or(0.0, |d| d)),
            close_time: values[6].as_i64().map_or(0, |v| v),
            quote_asset_volume: values[7].as_str().map_or(0.0, |v| v.parse::<f64>().map_or(0.0, |d| d)),
            upper_wick,
            lower_wick,
            real_body: body,
            color,
        }
    }
}

impl Candle {
    pub fn from_bybit(values: &[Value; 7], close_time: i64) -> (i64, Self) {
        let open = values[1].as_str().map_or(0.0, |v| v.parse::<f64>().map_or(0.0, |d| d));
        let high = values[2].as_str().map_or(0.0, |v| v.parse::<f64>().map_or(0.0, |d| d));
        let low = values[3].as_str().map_or(0.0, |v| v.parse::<f64>().map_or(0.0, |d| d));
        let close = values[4].as_str().map_or(0.0, |v| v.parse::<f64>().map_or(0.0, |d| d));
        let body = (open - close).abs();

        let (color, upper_wick, lower_wick) = match F64(open).cmp(&F64(close)) {
            Ordering::Less => {
                (CandleColor::Green, high - close, open - low)
            },
            Ordering::Equal => (CandleColor::None, high - close, open - low),
            Ordering::Greater => {
                (CandleColor::Red, high - open, close - low)
            }
        };

        let ct = values[0].as_str().map_or(0, |v| v.parse::<i64>().map_or(0, |d| d));

        (ct, Candle {
            open_time: ct,
            open,
            high: high.clone(),
            low: low.clone(),
            close,
            volume: values[5].as_str().map_or(0.0, |v| v.parse::<f64>().map_or(0.0, |d| d)),
            close_time,
            quote_asset_volume: values[6].as_str().map_or(0.0, |v| v.parse::<f64>().map_or(0.0, |d| d)),
            upper_wick,
            lower_wick,
            real_body: body,
            color,
        })
    }
}