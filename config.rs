use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_uri: String,
    pub num_threads: usize,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();
        let database_uri = env::var("DATABASE_URI").expect("DATABASE_URI is not set in .env file");
        let num_threads_str = env::var("NUM_THREADS").expect("NUM_THREADS is not set in .env file");
        let num_threads = num_threads_str.parse::<usize>().expect("NUM_THREADS must be a positive integer");
        Ok(Config {
            database_uri,
            num_threads,
        })
    }
}