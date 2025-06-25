use std::io;
use crate::data::SensorReading;

pub fn get_sensor_data() -> Vec<SensorReading> {
  vec![
    SensorReading {
      neighborhood: String::from("Centro"),
      rainfall_mm: 120.0,
      smoke_ppm: 10.0,
      temperature_c: 28.0,
    },
    SensorReading {
      neighborhood: String::from("Jardim das Flores"),
      rainfall_mm: 15.0,
      smoke_ppm: 85.0,
      temperature_c: 30.0,
    },
    SensorReading {
      neighborhood: String::from("São Pedro"),
      rainfall_mm: 0.0,
      smoke_ppm: 2.0,
      temperature_c: 23.0,
    },
    SensorReading {
      neighborhood: String::from("Santa Luzia"),
      rainfall_mm: 80.0,
      smoke_ppm: 50.0,
      temperature_c: 42.0,
    },
    SensorReading {
      neighborhood: String::from("Jardim Califórnia"),
      rainfall_mm: 140.0,
      smoke_ppm: 20.0,
      temperature_c: 42.0,
    },
  ]
}

pub fn read_user_input() -> SensorReading {
  fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Erro na leitura");
    buffer.trim().to_string()
  }

  let neighborhood = read_input("Digite o nome do bairro:");

  let rainfall_mm: f64 = read_input("Quantidade de chuva (em mm):")
    .parse()
    .expect("Digite um número válido");

  let smoke_ppm: f64 = read_input("Quantidade de fumaça (em ppm):")
    .parse()
    .expect("Digite um número válido");

  let temperature_c: f64 = read_input("Temperatura (em °C):")
    .parse()
    .expect("Digite um número válido");

  
  SensorReading {
    neighborhood,
    rainfall_mm,
    smoke_ppm,
    temperature_c,
  }
} 