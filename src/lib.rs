use crate::config::{init_env_config, Env};
use once_cell::sync::{Lazy, OnceCell};
use sqlx::{Pool, Postgres};

pub mod cache;
pub mod indexer;
pub mod pusher;
pub mod server;

pub mod config;

pub const INDEXER: &str = "near-indexer-s3";
pub const HTTP_INDEXER: &str = "http";

pub static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();
pub static PROJECT_CONFIG: Lazy<Env> = Lazy::new(init_env_config);

pub fn db_pool() -> &'static Pool<Postgres> {
    DB_POOL.get().expect("Get db pool fail")
}
