use chrono::Local;
use sqlx::{Pool, Sqlite};
use std::env;
use termion::color;

pub mod database;


#[derive(Clone)]
pub struct EnvVars {
    pub db_url: String,
}


#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<Sqlite>,
    pub env_vars: EnvVars,
}

// used to print log to stdout with time
pub fn log(input: &str) {
    let time = Local::now()
        .format("%y/%m/%d %T%.3f")
        .to_string();
    // Print time with yellow and input with white (can be changed if a colored string is passed)
    println!("{}{} {} {}", color::Fg(color::Rgb(64, 64, 64)), time,  color::Fg(color::White), input)
}


// used to print log to stderr with time
pub fn log_err(input: &str) {
    let time = Local::now()
        .format("%y/%m/%d %T%.3f")
        .to_string();
    // Print time with yellow and input with white (can be changed if a colored string is passed)
    eprintln!("{}{} {} {}", color::Fg(color::Rgb(200, 64, 64)), time,  color::Fg(color::White), input)
}


// retrieves environment variables
pub async fn get_env() -> EnvVars {
    let db_url = env::var("DB_URL");
    let db_url = match db_url {
        Ok(x) => {
            log("Got the DB_URL from environment variables");
            x
        },
        Err(e) => {
            panic!("Did not find DB_URL in environment variables    {}", e);
        },
    };
    
    EnvVars {
        db_url,
    }
}
