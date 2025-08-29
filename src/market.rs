use eq_float::F64;
use crate::{Candle, Trend, Wave, Zone, ZoneType};


//impl BybitApi for Market {}

#[derive(Debug, Clone, Default)]
pub struct Market {
    pub api_base: String,
    pub api_key: String,
    pub secret_key: String,
    pub symbol: String,
    pub quantity: String,

    pub last_zone: Zone,
    pub previous_wave: Wave,
    pub level_zero_current_trend: Trend,
    pub level_zero_trends: Vec<Trend>,
    pub level_one_current_trend: Trend,
    pub level_one_trends: Vec<Trend>,
    pub level_two_current_trend: Trend,
    pub level_two_trends: Vec<Trend>
}

impl Market {
    pub fn new(api_base: String, api_key: String, secret_key: String, symbol: String, quantity: String) -> Market {
        let mut market = Market::default();

        market.api_base = api_base;
        market.api_key = api_key;
        market.secret_key = secret_key;
        market.symbol = symbol;
        market.quantity = quantity;

        market
    }

    pub fn connect_zone(&mut self, new_zone: &Zone, candles: &mut Vec<Candle>, trade_candle: Option<&Candle>) {        
        match (&mut self.last_zone.zone_type, &new_zone.zone_type) {
            (ZoneType::None, ZoneType::None) => {}
            (ZoneType::None, _) => {
                self.last_zone = new_zone.to_owned();
            }
            (ZoneType::BearishRejBlock | ZoneType::Resistance, ZoneType::BullishRejBlock | ZoneType::Support) => {
                candles.retain(|candle| candle.open_time > self.last_zone.id);
                self.got_new_wave(
                    Wave::DownWave {
                        high_time: self.last_zone.id,
                        high: self.last_zone.start,
                        low_time: new_zone.id,
                        low: new_zone.start
                    },
                    trade_candle
                );
                self.last_zone = new_zone.to_owned();
            }
            (ZoneType::BullishRejBlock | ZoneType::Support, ZoneType::BearishRejBlock | ZoneType::Resistance) => {
                candles.retain(|candle| candle.open_time > self.last_zone.id);
                self.got_new_wave(
                    Wave::UpWave {
                        low_time: self.last_zone.id,
                        low: self.last_zone.start,
                        high_time: new_zone.id,
                        high: new_zone.start
                    },
                    trade_candle
                );
                self.last_zone = new_zone.to_owned();
            }
            (ZoneType::BearishRejBlock | ZoneType::Resistance, ZoneType::BearishRejBlock | ZoneType::Resistance) => {
                if self.last_zone.start < new_zone.start {
                    let id = self.last_zone.id;
                    match candles.iter().filter(|candle| candle.open_time > self.last_zone.id).min_by(|a, b| F64(a.low).cmp(&F64(b.low))) {
                        Some(min) => {
                            self.got_new_wave(Wave::DownWave {
                                high_time: self.last_zone.id,
                                high: self.last_zone.start,
                                low_time: min.open_time,
                                low: min.low
                            },trade_candle);
                            self.got_new_wave(Wave::UpWave {
                                low_time: min.open_time,
                                low: min.low,
                                high_time: new_zone.id,
                                high: new_zone.start
                            },trade_candle);
                            self.last_zone = new_zone.to_owned();
                        }
                        None => {
                            self.last_zone.start = new_zone.start;
                        }
                    }

                    candles.retain(|candle| candle.open_time > id);
                }
            }
            (ZoneType::BullishRejBlock | ZoneType::Support, ZoneType::BullishRejBlock | ZoneType::Support) => {
                if self.last_zone.start > new_zone.start {
                    let id = self.last_zone.id;
                    match candles.iter().filter(|candle| candle.open_time > self.last_zone.id).max_by(|a, b| F64(a.high).cmp(&F64(b.high))) {
                        Some(max) => {
                            self.got_new_wave(Wave::UpWave {
                                low_time: self.last_zone.id,
                                low: self.last_zone.start,
                                high_time: max.open_time,
                                high: max.high
                            },trade_candle);
                            self.got_new_wave( Wave::DownWave {
                                high_time: max.open_time,
                                high: max.high,
                                low_time: new_zone.id,
                                low: new_zone.start
                            },trade_candle);
                            self.last_zone = new_zone.to_owned();
                        }
                        None => {
                            self.last_zone.start = new_zone.start;
                        }
                    }

                    candles.retain(|candle| candle.open_time > id);
                }
            }
            _ => {}
        }
    }


    fn got_new_wave(&mut self, new_wave: Wave, trade_candle: Option<&Candle>) {
        /*match &self.level_zero_current_trend {
            Trend::None => {
                match &new_wave {
                    Wave::None => {}
                    Wave::UpWave { low_time, low, high_time, high } => {
                        self.level_zero_current_trend.swap_trend(&mut Trend::UpTrend {low_time: *low_time, low: *low, high: *high, high_time: *high_time, wave_one: new_wave.to_owned(), wave_two: Wave::None});
                    }
                    Wave::DownWave { high_time, high, low_time, low } => {
                        self.level_zero_current_trend.swap_trend(&mut Trend::DownTrend {high_time: *high_time, high: *high, low: *low, low_time: *low_time, wave_one: new_wave.to_owned(), wave_two: Wave::None});
                    }
                }
            }
            Trend::UpTrend {low_time:_, low, high, high_time, wave_one, wave_two} => {
                match &wave_one {
                    Wave::UpWave { low_time, low:_, high_time:_, high:_ } => {
                        match &wave_two {
                            Wave::None => {
                                match &new_wave {
                                    Wave::DownWave { high_time:_, high:_, low_time: new_low_time, low: new_low } => {
                                        match new_low <= low {
                                            true => {
                                                self.level_zero_trends.push(self.level_zero_current_trend.to_owned());
                                                external_trend(&mut self.level_one_current_trend, &mut self.level_one_trends, &mut self.level_two_current_trend, &mut self.level_two_trends, self.level_zero_current_trend.wave());
                                                self.level_zero_current_trend.swap_trend(&mut Trend::DownTrend { high_time: *high_time, high: *high, low: *new_low, low_time: *new_low_time, wave_one: new_wave.to_owned(), wave_two: Wave::None});
                                            }
                                            false => {
                                                self.level_zero_current_trend.update_wave_two(new_wave.to_owned());
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            Wave::DownWave { high_time:_, high:_, low_time: two_low_time, low: two_low } => {
                                match &new_wave {
                                    Wave::UpWave { low_time:_, low:_, high_time: new_high_time, high: new_high } => {
                                        match new_high > high {
                                            true => {
                                                let up_wave = Wave::UpWave { low_time: *low_time, low: *low, high_time: *new_high_time, high: *new_high };
                                                self.level_zero_current_trend.swap_trend(&mut Trend::UpTrend { low_time: *low_time, low: *low, high_time: *new_high_time, high: *new_high, wave_one: up_wave, wave_two: Wave::None});
                                            }
                                            false => {
                                                let new_up_trend = Trend::UpTrend {low_time: *low_time, low: *low, high: *high, high_time: *high_time, wave_one: wave_one.to_owned(), wave_two: Wave::None};
                                                self.level_zero_trends.push(new_up_trend);
                                                external_trend(&mut self.level_one_current_trend, &mut self.level_one_trends, &mut self.level_two_current_trend, &mut self.level_two_trends, Wave::UpWave {low_time: *low_time, low: *low, high: *high, high_time: *high_time});
                                                self.level_zero_current_trend.swap_trend(&mut Trend::DownTrend { high: *high, high_time: *high_time, low: *two_low, low_time: *two_low_time, wave_one: wave_two.to_owned(), wave_two: new_wave.to_owned()});
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            Wave::UpWave { low_time:_, low:_, high_time:_, high:_ } => {}
                        }
                    }
                    _ => {}
                }
            }
            Trend::DownTrend {high_time, high, low, low_time, wave_one, wave_two} => {
                match &wave_one {
                    Wave::DownWave { high_time:_, high:_, low_time:_, low:_ } => {
                        match wave_two {
                            Wave::None => {
                                match &new_wave {
                                    Wave::UpWave { low_time:_, low:_, high_time: new_high_time, high: new_high } => {
                                        match new_high >= high {
                                            true => {
                                                self.level_zero_trends.push(self.level_zero_current_trend.to_owned());
                                                external_trend(&mut self.level_one_current_trend, &mut self.level_one_trends, &mut self.level_two_current_trend, &mut self.level_two_trends, self.level_zero_current_trend.wave());
                                                self.level_zero_current_trend.swap_trend(&mut Trend::UpTrend {low_time: *low_time, low: *low, high: *new_high, high_time: *new_high_time, wave_one: new_wave.to_owned(), wave_two: Wave::None});
                                            }
                                            false => {
                                                self.level_zero_current_trend.update_wave_two(new_wave.to_owned());
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            Wave::UpWave { low_time:_, low:_, high_time: two_high_time, high: two_high} => {
                                match &new_wave {
                                    Wave::DownWave { high_time:_, high:_, low_time: new_low_time, low: new_low } => {
                                        match new_low < low {
                                            true => {
                                                let down_wave = Wave::DownWave { high_time: *high_time, high: *high, low_time: *low_time, low: *new_low };
                                                self.level_zero_current_trend.swap_trend(&mut Trend::DownTrend {high_time: *high_time, high: *high, low: *new_low, low_time: *new_low_time, wave_one: down_wave, wave_two: Wave::None});
                                            }
                                            false => {
                                                let new_down_trend = Trend::DownTrend {high_time: *high_time, high: *high, low: *low, low_time: *low_time, wave_one: wave_one.to_owned(), wave_two: Wave::None};
                                                external_trend(&mut self.level_one_current_trend, &mut self.level_one_trends, &mut self.level_two_current_trend, &mut self.level_two_trends, Wave::DownWave {high_time: *high_time, high: *high, low: *low, low_time: *low_time});
                                                self.level_zero_trends.push(new_down_trend);
                                                self.level_zero_current_trend.swap_trend(&mut Trend::UpTrend {low_time: *low_time, low: *low, high: *two_high, high_time: *two_high_time, wave_one: wave_two.to_owned(), wave_two: new_wave.to_owned()});
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }*/

        //self.analyse(trade_candle, &new_wave);
        self.previous_wave = new_wave;
    }

    /*fn analyse(&self, trade_candle: Option<&Candle>, current_wave: &Wave) {

        match &self.level_two_current_trend {
            Trend::DownTrend { high_time:_, high, low: two_low, low_time:_, wave_one:_, wave_two:_ } => {
                match &self.level_one_current_trend {
                    Trend::DownTrend {high_time:_, high:_, low: cur_low, low_time:_, wave_one:_, wave_two: Wave::None} => {
                        match (&self.previous_wave, &current_wave) {
                            (Wave::UpWave { low_time:_, low: w_low, high_time:_, high:_}, Wave::DownWave { high_time:_, high:_, low_time:_, low: d_low}) => {
                                if cur_low > two_low && cur_low == w_low &&  w_low < d_low {
                                    match trade_candle {
                                        Some(t_candle) => {
                                            if *high > t_candle.close {
                                                let mut body = BybitTakeprofitStoplost::new_buy_body(&self.symbol, &self.quantity, t_candle.close, *high);
                                                let _ = self.new_takeprofit_stoplost_order(&self.api_base, &mut body, &self.api_key, &self.secret_key);
                                            }
                                        }
                                        None => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }*/
}
/*

fn external_trend(level_one_current_trend: &mut Trend, level_one_trends: &mut Vec<Trend>, level_two_current_trend: &mut Trend, level_two_trends: &mut Vec<Trend>, new_wave: Wave) {
    match &level_one_current_trend {
        Trend::None => {
            match &new_wave {
                Wave::None => {}
                Wave::UpWave { low_time, low, high_time, high } => {
                    level_one_current_trend.swap_trend(&mut Trend::UpTrend {low_time: *low_time, low: *low, high: *high, high_time: *high_time, wave_one: new_wave.to_owned(), wave_two: Wave::None});
                }
                Wave::DownWave { high_time, high, low_time, low } => {
                    level_one_current_trend.swap_trend(&mut Trend::DownTrend {high_time: *high_time, high: *high, low: *low, low_time: *low_time, wave_one: new_wave.to_owned(), wave_two: Wave::None});
                }
            }
        }
        Trend::UpTrend {low_time:_, low, high, high_time, wave_one, wave_two} => {
            match &wave_one {
                Wave::UpWave { low_time, low:_, high_time:_, high:_ } => {
                    match &wave_two {
                        Wave::None => {
                            match &new_wave {
                                Wave::DownWave { high_time:_, high:_, low_time: new_low_time, low: new_low } => {
                                    match new_low <= low {
                                        true => {
                                            level_one_trends.push(level_one_current_trend.to_owned());
                                            external_two_trend(level_two_current_trend, level_two_trends, level_one_current_trend.wave());
                                            level_one_current_trend.swap_trend(&mut Trend::DownTrend { high_time: *high_time, high: *high, low: *new_low, low_time: *new_low_time, wave_one: new_wave.to_owned(), wave_two: Wave::None});
                                        }
                                        false => {
                                            level_one_current_trend.update_wave_two(new_wave.to_owned());
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        Wave::DownWave { high_time:_, high:_, low_time: two_low_time, low: two_low } => {
                            match &new_wave {
                                Wave::UpWave { low_time:_, low:_, high_time: new_high_time, high: new_high } => {
                                    match new_high > high {
                                        true => {
                                            let up_wave = Wave::UpWave { low_time: *low_time, low: *low, high_time: *new_high_time, high: *new_high };
                                            level_one_current_trend.swap_trend(&mut Trend::UpTrend { low_time: *low_time, low: *low, high_time: *new_high_time, high: *new_high, wave_one: up_wave, wave_two: Wave::None});
                                        }
                                        false => {
                                            let new_up_trend = Trend::UpTrend {low_time: *low_time, low: *low, high: *high, high_time: *high_time, wave_one: wave_one.to_owned(), wave_two: Wave::None};
                                            level_one_trends.push(new_up_trend);
                                            external_two_trend(level_two_current_trend, level_two_trends, Wave::UpWave {low_time: *low_time, low: *low, high: *high, high_time: *high_time});
                                            level_one_current_trend.swap_trend(&mut Trend::DownTrend { high: *high, high_time: *high_time, low: *two_low, low_time: *two_low_time, wave_one: wave_two.to_owned(), wave_two: new_wave.to_owned()});
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        Wave::UpWave { low_time:_, low:_, high_time:_, high:_ } => {}
                    }
                }
                _ => {}
            }
        }
        Trend::DownTrend {high_time, high, low, low_time, wave_one, wave_two} => {
            match &wave_one {
                Wave::DownWave { high_time:_, high:_, low_time:_, low:_ } => {
                    match wave_two {
                        Wave::None => {
                            match &new_wave {
                                Wave::UpWave { low_time:_, low:_, high_time: new_high_time, high: new_high } => {
                                    match new_high >= high {
                                        true => {
                                            level_one_trends.push(level_one_current_trend.to_owned());
                                            external_two_trend(level_two_current_trend, level_two_trends, level_one_current_trend.wave());
                                            level_one_current_trend.swap_trend(&mut Trend::UpTrend {low_time: *low_time, low: *low, high: *new_high, high_time: *new_high_time, wave_one: new_wave.to_owned(), wave_two: Wave::None});
                                        }
                                        false => {
                                            level_one_current_trend.update_wave_two(new_wave.to_owned());
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        Wave::UpWave { low_time:_, low:_, high_time: two_high_time, high: two_high} => {
                            match &new_wave {
                                Wave::DownWave { high_time:_, high:_, low_time: new_low_time, low: new_low } => {
                                    match new_low < low {
                                        true => {
                                            let down_wave = Wave::DownWave { high_time: *high_time, high: *high, low_time: *low_time, low: *new_low };
                                            level_one_current_trend.swap_trend(&mut Trend::DownTrend {high_time: *high_time, high: *high, low: *new_low, low_time: *new_low_time, wave_one: down_wave, wave_two: Wave::None});
                                        }
                                        false => {
                                            let new_down_trend = Trend::DownTrend {high_time: *high_time, high: *high, low: *low, low_time: *low_time, wave_one: wave_one.to_owned(), wave_two: Wave::None};
                                            external_two_trend(level_two_current_trend, level_two_trends, Wave::DownWave {high_time: *high_time, high: *high, low: *low, low_time: *low_time});
                                            level_one_trends.push(new_down_trend);
                                            level_one_current_trend.swap_trend(&mut Trend::UpTrend {low_time: *low_time, low: *low, high: *two_high, high_time: *two_high_time, wave_one: wave_two.to_owned(), wave_two: new_wave.to_owned()});
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

fn external_two_trend(level_two_current_trend: &mut Trend, level_two_trends: &mut Vec<Trend>, new_wave: Wave) {
    match &level_two_current_trend {
        Trend::None => {
            match &new_wave {
                Wave::None => {}
                Wave::UpWave { low_time, low, high_time, high } => {
                    level_two_current_trend.swap_trend(&mut Trend::UpTrend {low_time: *low_time, low: *low, high: *high, high_time: *high_time, wave_one: new_wave, wave_two: Wave::None});
                }
                Wave::DownWave { high_time, high, low_time, low } => {
                    level_two_current_trend.swap_trend(&mut Trend::DownTrend {high_time: *high_time, high: *high, low: *low, low_time: *low_time, wave_one: new_wave, wave_two: Wave::None});
                }
            }
        }
        Trend::UpTrend {low_time:_, low, high, high_time, wave_one, wave_two} => {
            match &wave_one {
                Wave::UpWave { low_time, low:_, high_time:_, high:_ } => {
                    match &wave_two {
                        Wave::None => {
                            match &new_wave {
                                Wave::DownWave { high_time:_, high:_, low_time: new_low_time, low: new_low } => {
                                    match new_low <= low {
                                        true => {
                                            level_two_trends.push(level_two_current_trend.to_owned());
                                            level_two_current_trend.swap_trend(&mut Trend::DownTrend { high_time: *high_time, high: *high, low: *new_low, low_time: *new_low_time, wave_one: new_wave, wave_two: Wave::None});
                                        }
                                        false => {
                                            level_two_current_trend.update_wave_two(new_wave);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        Wave::DownWave { high_time:_, high:_, low_time: two_low_time, low: two_low } => {
                            match &new_wave {
                                Wave::UpWave { low_time:_, low:_, high_time: new_high_time, high: new_high } => {
                                    match new_high > high {
                                        true => {
                                            let up_wave = Wave::UpWave { low_time: *low_time, low: *low, high_time: *new_high_time, high: *new_high };
                                            level_two_current_trend.swap_trend(&mut Trend::UpTrend { low_time: *low_time, low: *low, high_time: *new_high_time, high: *new_high, wave_one: up_wave, wave_two: Wave::None});
                                        }
                                        false => {
                                            let new_up_trend = Trend::UpTrend {low_time: *low_time, low: *low, high: *high, high_time: *high_time, wave_one: wave_one.to_owned(), wave_two: Wave::None};
                                            level_two_trends.push(new_up_trend);
                                            level_two_current_trend.swap_trend(&mut Trend::DownTrend { high: *high, high_time: *high_time, low: *two_low, low_time: *two_low_time, wave_one: wave_two.to_owned(), wave_two: new_wave.to_owned()});
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        Wave::UpWave { low_time:_, low:_, high_time:_, high:_ } => {}
                    }
                }
                _ => {}
            }
        }
        Trend::DownTrend {high_time, high, low, low_time, wave_one, wave_two} => {
            match &wave_one {
                Wave::DownWave { high_time:_, high:_, low_time:_, low:_ } => {
                    match wave_two {
                        Wave::None => {
                            match &new_wave {
                                Wave::UpWave { low_time:_, low:_, high_time: new_high_time, high: new_high } => {
                                    match new_high >= high {
                                        true => {
                                            level_two_trends.push(level_two_current_trend.to_owned());
                                            level_two_current_trend.swap_trend(&mut Trend::UpTrend {low_time: *low_time, low: *low, high: *new_high, high_time: *new_high_time, wave_one: new_wave, wave_two: Wave::None});
                                        }
                                        false => {
                                            level_two_current_trend.update_wave_two(new_wave);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        Wave::UpWave { low_time:_, low:_, high_time: two_high_time, high: two_high} => {
                            match &new_wave {
                                Wave::DownWave { high_time:_, high:_, low_time: new_low_time, low: new_low } => {
                                    match new_low < low {
                                        true => {
                                            let down_wave = Wave::DownWave { high_time: *high_time, high: *high, low_time: *low_time, low: *new_low };
                                            level_two_current_trend.swap_trend(&mut Trend::DownTrend {high_time: *high_time, high: *high, low: *new_low, low_time: *new_low_time, wave_one: down_wave, wave_two: Wave::None});
                                        }
                                        false => {
                                            let new_down_trend = Trend::DownTrend {high_time: *high_time, high: *high, low: *low, low_time: *low_time, wave_one: wave_one.to_owned(), wave_two: Wave::None};
                                            level_two_trends.push(new_down_trend);
                                            level_two_current_trend.swap_trend(&mut Trend::UpTrend {low_time: *low_time, low: *low, high: *two_high, high_time: *two_high_time, wave_one: wave_two.to_owned(), wave_two: new_wave.to_owned()});
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}*/