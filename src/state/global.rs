use std::sync::Mutex;

use super::promotion;

#[derive(Debug)]
pub struct Global {
    pub promotions: Mutex<Vec<promotion::State>>,
}

impl Global {
    pub fn new(promotions: &Vec<promotion::State>) -> Self {
        Self {
            promotions: Mutex::new(promotions.to_vec()),
        }
    }
}
