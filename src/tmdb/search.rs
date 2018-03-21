use std::collections::HashMap;

use reqwest::{Client, StatusCode};

use error;

#[derive(Debug, Deserialize)]
pub struct Movie {
    pub id: i64,
    pub title: String,
    pub original_title: String,
    pub overview: String,
    pub release_date: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Paged<T> {
    pub page: i32,
    pub total_pages: i32,
    pub total_results: i32,
    pub results: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct Error {
    pub status_message: String,
    pub status_code: i32,
}

static BASE_URL: &'static str = "https://api.themoviedb.org/3";
static API_KEY: &'static str = include_str!("API_KEY.txt");

pub fn movie(query: &str, year: Option<i32>) -> Result<Paged<Movie>, error::Error> {
    let url = format!("{}/search/movie", BASE_URL);
    let client = Client::new();

    let mut params: HashMap<&'static str, String> = HashMap::new();
    params.insert("api_key", API_KEY.trim().into());
    params.insert("query", query.into());
    if let Some(year) = year {
        params.insert("year", format!("{}", year));
    }

    let req = client.get(&url).query(&params).build()?;
    let mut resp = client.execute(req)?;

    match resp.status() {
        StatusCode::Ok => {
            let results: Paged<Movie> = resp.json()?;
            Ok(results)
        }
        _ => {
            let error: Error = resp.json()?;
            Err(error.into())
        }
    }
}
