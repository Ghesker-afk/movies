use crate::data::{RiskAlert, RiskType};

impl RiskAlert {
  pub fn format_message(&self) -> String {
    let risk_description = match self.risk {
      RiskType::Flood => "Risco de alagamento",
      RiskType::Fire => "Risco de incêndio",
      RiskType::Heatwave => "Risco de onda de calor",
    };

    format!(
      "[Nível {}] {} em {}!",
      self.level, risk_description, self.neighborhood
    )
  }
}