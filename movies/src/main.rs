#[derive(Debug)]
pub enum Genre {
    Action,
    Comedy,
    Drama,
    SciFi,
    Horror,
    Fantasy,
    Documentary,
}

#[derive(Debug)]
pub struct Movie {
    pub title: String,
    pub genre: Genre,
    pub rating: f32,
}

fn main() {
    println!("Hello, World!");
}
