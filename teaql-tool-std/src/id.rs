use nanoid::nanoid;
use ulid::Ulid;
use uuid::Uuid;

pub struct IdTool;

impl IdTool {
    pub fn new() -> Self {
        Self
    }

    pub fn uuid(&self) -> String {
        Uuid::new_v4().to_string()
    }

    pub fn uuid_v7(&self) -> String {
        Uuid::now_v7().to_string()
    }

    pub fn ulid(&self) -> String {
        Ulid::new().to_string()
    }

    pub fn nanoid(&self) -> String {
        nanoid!()
    }

    pub fn with_prefix(&self, prefix: &str) -> String {
        format!("{}_{}", prefix, self.nanoid())
    }
}

impl Default for IdTool {
    fn default() -> Self {
        Self::new()
    }
}
