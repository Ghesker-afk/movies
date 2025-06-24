use std::io;

fn read_line_trimmed() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler input");
    input.trim().to_string()
}

fn read_genres_from_input() -> Vec<Genre> {
    println!("Digite seus gêneros favoritos, separados por vírgula (ex: action, comedy):");
    let input = read_line_trimmed();

    let mut genres = Vec::new();
    let parts = input.split(',');

    for part in parts {
        let maybe_genre = Genre::from_str(part);
        if let Some(genre) = maybe_genre {
            genres.push(genre);
        }
    }

    genres
}

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

impl Genre {
    fn from_str(input: &str) -> Option<Genre> {
        let s = input.trim();
        if s == "action" {
            Some(Genre::Action)
        } else if s == "comedy" {
            Some(Genre::Comedy) 
        } else if s == "drama" {
            Some(Genre::Drama)
        } else if s == "scifi" {
            Some(Genre::SciFi)
        } else if s == "horror" {
            Some(Genre::Horror)
        } else if s == "fantasy" {
            Some(Genre::Fantasy)
        } else if s == "documentary" {
            Some(Genre::Documentary)
        } else {
            None
        }
    }
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

pub struct MultiCriteriaUser {
    pub preferred_genres: Vec<Genre>,
    pub minimum_rating: f32,
}

impl UserPreference for MultiCriteriaUser {
    fn likes(&self, movie: &Movie) -> bool {
        let mut genre_ok = false;

        for g in &self.preferred_genres {
            if *g == movie.genre {
                genre_ok = true;
            }
        }

        let rating_ok = movie.rating >= self.mininum_rating;

        genre_ok && rating_ok
    }
}

fn main() {
    let catalog = vec![
        Movie { title: "Explosão Final".to_string(), genre: Genre::Action, rating: 7.5 },
        Movie { title: "Risada Garantida".to_string(), genre: Genre::Comedy, rating: 6.2 },
        Movie { title: "Drama Profundo".to_string(), genre: Genre::Drama, rating: 8.9 },
        Movie { title: "Ficção Brilhante".to_string(), genre: Genre::SciFi, rating: 9.0 },
    ];

    let preferred_genres = read_genres_from_input();
}
