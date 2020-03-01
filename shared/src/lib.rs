extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RandomResponse {
    pub number: i32,
}
