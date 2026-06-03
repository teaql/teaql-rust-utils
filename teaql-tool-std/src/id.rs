use nanoid::nanoid;
use ulid::Ulid;
use uuid::Uuid;
use teaql_tool_core::MustComment;

pub struct IdTool;

impl IdTool {
    pub fn new() -> Self {
        Self
    }

    pub fn uuid(&self) -> MustComment<String> {
        MustComment::new(Uuid::new_v4().to_string())
    }

    pub fn uuid_v7(&self) -> MustComment<String> {
        MustComment::new(Uuid::now_v7().to_string())
    }

    pub fn ulid(&self) -> MustComment<String> {
        MustComment::new(Ulid::new().to_string())
    }

    pub fn nanoid(&self) -> MustComment<String> {
        MustComment::new(nanoid!())
    }

    pub fn with_prefix(&self, prefix: &str) -> MustComment<String> {
        MustComment::new(format!("{}_{}", prefix, self.nanoid().comment("internal extraction")))
    }
}

impl Default for IdTool {
    fn default() -> Self {
        Self::new()
    }
}
