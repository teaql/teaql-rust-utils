use nanoid::nanoid;
use ulid::Ulid;
use uuid::Uuid;
use teaql_tool_core::MustPurpose;

pub struct IdTool;

impl IdTool {
    pub fn new() -> Self {
        Self
    }

    pub fn uuid(&self) -> MustPurpose<String> {
        MustPurpose::new(Uuid::new_v4().to_string())
    }

    pub fn uuid_v7(&self) -> MustPurpose<String> {
        MustPurpose::new(Uuid::now_v7().to_string())
    }

    pub fn ulid(&self) -> MustPurpose<String> {
        MustPurpose::new(Ulid::new().to_string())
    }

    pub fn nanoid(&self) -> MustPurpose<String> {
        MustPurpose::new(nanoid!())
    }

    pub fn with_prefix(&self, prefix: &str) -> MustPurpose<String> {
        MustPurpose::new(format!("{}_{}", prefix, self.nanoid().purpose("internal extraction")))
    }
}

impl Default for IdTool {
    fn default() -> Self {
        Self::new()
    }
}
