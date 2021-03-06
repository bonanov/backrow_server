use lazy_static::lazy_static;
use std::env::var;

lazy_static! {
    pub static ref APP_ADDR: String =
        var("APP_ADDR").unwrap_or_else(|_| String::from("127.0.0.1:10008"));
    pub static ref APP_DOMAIN: String =
        var("APP_DOMAIN").unwrap_or_else(|_| self::APP_ADDR.clone());
    pub static ref APP_ORIGIN: String = format!("://{}", self::APP_DOMAIN.clone());
    pub static ref DATABASE_URL: String = var("DATABASE_URL").expect("DATABASE_URL");
    pub static ref DISCORD_CLIENT_ID: String = var("DISCORD_CLIENT_ID").unwrap_or_default();
    pub static ref DISCORD_CLIENT_SECRET: String = var("DISCORD_CLIENT_SECRET").unwrap_or_default();
    pub static ref DISCORD_REDIRECT_URL: String = var("DISCORD_REDIRECT_URL").unwrap_or_default();
}
