use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        Config {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            jwt_expiration: env::var("JWT_EXPIRATION")
                .expect("JWT_EXPIRATION must be set")
                .parse()
                .expect("JWT_EXPIRATION must be a number"),
        }
    }
} 