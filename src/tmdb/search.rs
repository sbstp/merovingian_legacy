use std::collections::HashMap;

use serde::Deserialize;
use reqwest::Client;

#[derive(Debug, Deserialize)]
pub struct Movie {
    id: i64,
    title: String,
    original_title: String,
    overview: String,
    release_date: String,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Paged<T> {
    page: i32,
    results: Vec<T>,
    total_pages: i32,
    total_results: i32,
}

static BASE_URL: &'static str = "https://api.themoviedb.org/3";
static API_KEY: &'static str = include_str!("API_KEY.txt");

pub fn movie(query: &str, year: Option<i32>) {
    let url = format!("{}/search/movie", BASE_URL);
    let client = Client::new();
    let mut params = HashMap::new();
    params.insert("api_key", API_KEY.trim());
    params.insert("query", query);
    let mut req = client.get(&url).query(&params).build().unwrap();
    println!("{}", req.url());
    let mut resp = client.execute(req).unwrap();
    //.send().unwrap();
    // let text = resp.text().unwrap();
    let results: Paged<Movie> = resp.json().unwrap();
    println!("{:#?}", results);
}
