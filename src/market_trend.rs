
#[derive(Debug, Clone, Default)]
pub enum MarketTrend {
    #[default]
    None,
    UpTrend,
    DownTrend
}

#[derive(Debug, Clone)]
pub enum MarketEvent {
    BosUp,
    BosDown,
    SweepUp,
    SweepDown
}

#[derive(Debug, Clone, Default)]
pub struct ExternalMarketTrend {
    pub external_high: f64,
    pub external_low: f64,
    pub external_events: Vec<MarketEvent>,
    pub external_trend: MarketTrend,
    pub internal_high: f64,
    pub internal_low: f64,
    pub internal_events: Vec<MarketEvent>,
    pub internal_trend: MarketTrend,
    pub sweep_external_high: Option<f64>,
    pub sweep_external_low: Option<f64>
}