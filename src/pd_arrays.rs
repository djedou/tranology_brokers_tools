use crate::{Candle, Market, Zone, ZoneType};



pub trait PdArrays {
    fn pd_arrays(market: &mut Market, candles: &mut Vec<Candle>, candle_one: &Candle, candle_two: &Candle, candle_three: &Candle, candle_four: &Candle, trade_candle: Option<&Candle>) {
        Self::swing_high_low(market, candles, candle_two, candle_three, candle_four, trade_candle);
        Self::support(market, candles, candle_one, candle_two, candle_three, candle_four, trade_candle);
        Self::resistance(market, candles, candle_one, candle_two, candle_three, candle_four, trade_candle);
    }

    fn swing_high_low(market: &mut Market, candles: &mut Vec<Candle>, candle_two: &Candle, candle_three: &Candle, candle_four: &Candle, trade_candle: Option<&Candle>) {
        let candle_three_close_limit = if candle_three.is_green() {candle_three.close} else {candle_three.open};
        
        if candle_three.high >= candle_two.high && candle_three.high >= candle_four.high && candle_three.upper_wick > 0.0
        {
            let zone = Zone::new(
                candle_three.open_time,
                candle_three.high,
                candle_three_close_limit,
                ZoneType::BearishRejBlock
            );
            market.connect_zone(&zone, candles, trade_candle);
        }

        let candle_three_open_limit = if candle_three.is_green() {candle_three.open} else {candle_three.close};

        if candle_three.low <= candle_two.low && candle_three.low <= candle_four.low && candle_three.lower_wick > 0.0
        {
            let zone = Zone::new(
                candle_three.open_time,
                candle_three.low,
                candle_three_open_limit,
                ZoneType::BullishRejBlock
            );
            market.connect_zone(&zone, candles, trade_candle);
        }
    }

    fn support(market: &mut Market, candles: &mut Vec<Candle>, candle_one: &Candle, candle_two: &Candle, candle_three: &Candle, candle_four: &Candle, trade_candle: Option<&Candle>) {
        if candle_three.is_green() 
            && candle_two.is_red()
            && candle_two.low == candle_three.low
            && candle_two.close == candle_three.open
            && candle_three.open <= candle_four.low
            && candle_two.close < candle_one.low
        {
            let zone = Zone::new(
                candle_three.open_time,
                candle_three.low,
                candle_three.low,
                ZoneType::Support
            );
            market.connect_zone(&zone, candles, trade_candle);
        }
    }

    fn resistance(market: &mut Market, candles: &mut Vec<Candle>, candle_one: &Candle, candle_two: &Candle, candle_three: &Candle, candle_four: &Candle, trade_candle: Option<&Candle>) {
        if candle_three.is_red() 
            && candle_two.is_green()
            && candle_two.high == candle_three.high
            && candle_two.close == candle_three.open
            && candle_three.open >= candle_four.high
            && candle_two.close > candle_one.high
        {
            let zone = Zone::new(
                candle_three.open_time,
                candle_three.high,
                candle_three.high,
                ZoneType::Resistance
            );
            market.connect_zone(&zone, candles, trade_candle);
        }
    }
}