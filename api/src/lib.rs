#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rust_i18n;
pub mod cookies;
pub mod db;
pub mod models;
pub mod pagination;
pub mod request;
pub mod response;
pub mod routes;
pub mod schema;
pub mod services;
pub mod storage;
pub mod types;
pub mod utils;

i18n!("locales");
