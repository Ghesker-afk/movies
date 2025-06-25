#[derive(Debug)]
pub enum RiskType {
  Flood,
  Fire,
  Heatwave,
}

#[derive(Debug)]
pub struct SensorReading {
  pub neighborhood: String,
  pub rainfall_mm: f64,
  pub smoke_ppm: f64,
  pub temperature_c: f64,
}

#[derive(Debug)]
pub struct RiskAlert {
  pub neighborhood: String,
  pub risk: RiskType,
  pub level: u8, // 1 (baixo) a 5 (crítico)
}