use dotenvy::dotenv;
use std::env;
use crate::models::errors::config_errors::{EnvVarError, EnvVarErrorKind};

pub struct AppConfig {
    pub database_url: String,
    pub url: String,
    pub port: u16,
    pub jwt_secret: String,
    pub token_create_user: String,
}

pub fn load_config() -> Result<AppConfig, EnvVarError> {
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => {
            return Err(EnvVarError::new(
                1001,
                "No se encontró en el archivo .env",
                EnvVarErrorKind::Missing,
                "DATABASE_URL",
                "env.rs::load_config"
            ));
        }
    };

    let url = match env::var("URL") {
        Ok(val) => val,
        Err(_) => {
            return Err(EnvVarError::new(
                1002,
                "No se encontró en el archivo .env",
                EnvVarErrorKind::Missing,
                "URL",
                "env.rs::load_config"
            ));
        }
    };

    let port = match env::var("PORT") {
        Ok(val) => match val.parse::<u16>() {
            Ok(p) => p,
            Err(_) => {
                return Err(EnvVarError::new(
                    1003,
                    "El valor no es un número válido",
                    EnvVarErrorKind::Invalid,
                    "PORT",
                    "env.rs::load_config"
                ));
            }
        },
        Err(_) => {
            return Err(EnvVarError::new(
                1004,
                "No se encontró en el archivo .env",
                EnvVarErrorKind::Missing,
                "PORT",
                "env.rs::load_config"
            ));
        }
    };

    let jwt_secret = match env::var("JWT_SECRET") {
        Ok(val) => val,
        Err(_) => {
            return Err(EnvVarError::new(
                1005,
                "No se encontró en el archivo .env",
                EnvVarErrorKind::Missing,
                "JWT_SECRET",
                "env.rs::load_config"
            ));
        }
    };

    let token_create_user = match env::var("TOKEN_CREATE_USER") {
        Ok(val) => val,
        Err(_) => {
            return Err(EnvVarError::new(
                1006,
                "No se encontró en el archivo .env",
                EnvVarErrorKind::Missing,
                "TOKEN_CREATE_USER",
                "env.rs::load_config"
            ));
        }
    };

    Ok(AppConfig { 
        database_url, 
        url,
        port,
        jwt_secret,
        token_create_user,
    })
}

pub fn load_env() {
    dotenv().ok();
}
