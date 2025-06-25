mod data;
mod logic;
mod alerts;
mod sensor;

use sensor::{get_sensor_data, read_user_input};
use logic::analyze_risks;

fn main() {
    println!("Sistema de Alerta Ambiental iniciado.");

    println!("Deseja usar dados simulado(s) ou digitar manualmente(m)?");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Erro");
    let choice = choice.trim();

    let readings = if choice == "m" {
        let input = read_user_input();
        vec![input]
    } else {
        get_sensor_data()
    };

    let alerts = analyze_risks(&readings);

    println!("\nAlertas detectados:");
    for alert in &alerts {
        println!("{}", alert.format_message());
    }
}
