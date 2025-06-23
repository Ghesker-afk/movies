#[derive(Debug, Clone)]
pub enum Genre {
    Action,
    Comedy,
    Drama,
    SciFi,
    Horror,
    Fantasy,
    Documentary,
}

#[derive(Debug, Clone)]
pub struct Movie {
    pub title: String,
    pub genre: Genre,
    pub rating: f32,
}

pub trait UserPreference {
    fn likes(&self, movie: &Movie) -> bool;
}

pub struct ActionFan;

impl UserPreference for ActionFan {
    fn likes(&self, movie: &Movie) -> bool {
        matches!(movie.genre, Genre::Action)
    }
}

pub struct ComedyLover;

impl UserPreference for ComedyLover {
    fn likes(&self, movie: &Movie) -> bool {
        matches!(movie.genre, Genre::Comedy)
    }
}

pub struct PickyCritic {
    pub minimum_rating: f32,
}

impl UserPreference for PickyCritic {
    fn likes(&self, movie: &Movie) -> bool {
        let nota_do_filme = movie.rating;
        let nota_minima = self.minimum_rating;

        let atende_exigencia = nota_do_filme >= nota_minima;
        atende_exigencia
    }
}

pub fn recommend_movies<T: UserPreference>(user: &T, catalog: &[Movie]) -> Vec<Movie> {
    let mut result = Vec::new();

    for movie in catalog {
        if user.likes(movie) {
            result.push(movie.clone());
        }
    }

    result
}

fn main() {
    println!("Hello, World!");
}
