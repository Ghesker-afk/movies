use crate::data::{SensorReading, RiskAlert, RiskType};

pub fn analyze_risks(readings: &[SensorReading]) -> Vec<RiskAlert> {
  let mut alerts = Vec::new();

  for reading in readings {
    // Detecta risco de alagamento
    if reading.rainfall_mm > 100.0 {
      alerts.push(RiskAlert {
        neighborhood: reading.neighborhood.clone(),
        risk: RiskType::Flood,
        level: 4,
      });
    }

    // Detecta risco de incêndio
    if reading.smoke_ppm > 80.0 {
      alerts.push(RiskAlert {
        neighborhood: reading.neighborhood.clone(),
        risk: RiskType::Fire,
        level: 5,
      });
    }

    // Detecta risco de onda de calor
    if reading.temperature_c > 40.0 {
      alerts.push(RiskAlert {
        neighborhood: reading.neighborhood.clone(),
        risk: RiskType::Heatwave,
        level: 3,
      });
    }
  }

  alerts
}