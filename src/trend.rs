use crate::Wave;
use std::mem::swap;

#[derive(Debug, Clone, Default)]
pub enum Trend {
    #[default]
    None,
    UpTrend {
        low_time: i64,
        low: f64,
        high: f64,
        high_time: i64,
        wave_one: Wave,
        wave_two: Wave
    },
    DownTrend {
        high_time: i64,
        high: f64,
        low: f64,
        low_time: i64,
        wave_one: Wave,
        wave_two: Wave
    }
}

impl Trend {
    pub fn swap_trend(&mut self, new_trend: &mut Trend) {
        swap(self, new_trend);
    }

    pub fn update_wave_two(&mut self, wave: Wave) {
        match self {
            Trend::UpTrend {low_time:_, low:_, high:_, high_time:_, wave_one:_, wave_two} => {
                *wave_two = wave;
            },
            Trend::DownTrend {high_time:_, high:_, low:_, low_time:_, wave_one:_, wave_two} => {
                *wave_two = wave;
            }
            _ => {}
        }
    }

    pub fn wave(&self) -> Wave {
        match &self {
            Trend::UpTrend {low_time, low, high, high_time, wave_one:_, wave_two:_} => {
                Wave::UpWave { low_time: *low_time, low: *low, high_time: *high_time, high: *high }
            },
            Trend::DownTrend {high_time, high, low, low_time, wave_one:_, wave_two:_} => {
                Wave::DownWave { high_time: *high_time, high: *high, low_time: *low_time, low: *low }
            }
            _ => Wave::None
        }
    }

}