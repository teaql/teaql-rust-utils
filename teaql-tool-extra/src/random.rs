use rand::RngExt;

pub struct RandomTool;

impl RandomTool {
    pub fn new() -> Self {
        Self
    }

    pub fn int(&self, min: i64, max: i64) -> i64 {
        let mut rng = rand::rng();
        rng.random_range(min..=max)
    }

    pub fn float(&self, min: f64, max: f64) -> f64 {
        let mut rng = rand::rng();
        rng.random_range(min..=max)
    }

    pub fn boolean(&self) -> bool {
        let mut rng = rand::rng();
        rng.random_bool(0.5)
    }
}

impl Default for RandomTool {
    fn default() -> Self {
        Self::new()
    }
}
