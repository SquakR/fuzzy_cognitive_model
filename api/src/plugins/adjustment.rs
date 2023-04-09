pub mod models;
pub mod routes;
pub mod services;
pub mod types;

use super::Plugin;

pub struct AdjustmentPlugin;

impl Plugin for AdjustmentPlugin {
    fn get_name(&self) -> String {
        String::from("Adjustment With Genetic Algorithms")
    }
}
