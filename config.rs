use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub database_connection_string: String,
    pub thread_pool_size: usize,
}

impl Configuration {
    pub fn load_from_environment() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv::dotenv().ok();
        
        let database_connection_string = env::var("DATABASE_URI")
            .expect("DATABASE_URI environment variable is not set in .env file");
        
        let thread_pool_size_str = env::var("NUM_THREADS")
            .expect("NUM_THREADS environment variable is not set in .env file");
        
        let thread_pool
    
        Size = thread_pool_size_str.parse::<usize>()
            .expect("NUM_THREADS environment variable must be a positive integer");
        
        Ok(Configuration {
            database_connection_string,
            thread_pool_size,
        })
    }
}