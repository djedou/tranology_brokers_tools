


#[derive(Debug, Clone, Default)]
pub enum Wave {
    #[default]
    None,
    UpWave {
        low_time: i64,
        low: f64,
        high_time: i64,
        high: f64
    },
    DownWave {
        high_time: i64,
        high: f64,
        low_time: i64,
        low: f64
    }
}

impl Wave {
    pub fn get_high(&self) -> Option<f64> {
        match  &self {
            Wave::DownWave { high_time:_, high, low_time:_, low:_ } => Some(*high),
            Wave::UpWave { low_time:_, low:_, high_time:_, high } => Some(*high),
            Wave::None => None
        }
    }

    pub fn get_low(&self) -> Option<f64> {
        match  &self {
            Wave::DownWave { high_time:_, high:_, low_time:_, low } => Some(*low),
            Wave::UpWave { low_time:_, low, high_time:_, high:_ } => Some(*low),
            Wave::None => None
        }
    }
}