use crate::macros::*;

use teaql_tool_std::json::JsonTool;
use serde_json::Value;
use json_patch::Patch;
use teaql_tool_core::{MustPurpose, Result};

define_context_facade!("std", json, ContextJsonExt, ContextJsonFacade);

#[cfg(feature = "std")]
impl<'a> ContextJsonFacade<'a> {
    delegate_comment! { JsonTool::new(),
        fn has(&self, v: &Value, pointer: &str) -> bool;
        fn merge(&self, a: &Value, b: &Value) -> Value;
        fn diff(&self, a: &Value, b: &Value) -> Patch
    }
    delegate_res_comment! { JsonTool::new(),
        fn parse(&self, s: &str) -> Value;
        fn stringify(&self, v: &Value) -> String;
        fn stringify_pretty(&self, v: &Value) -> String
    }

    // Methods with lifetime parameters or &mut Value must be manually implemented

    pub fn get<'v>(&self, v: &'v Value, pointer: &str) -> Option<&'v Value> {
        JsonTool::new().get(v, pointer)
    }

    pub fn set(&self, v: &mut Value, pointer: &str, value: Value) -> Result<()> {
        JsonTool::new().set(v, pointer, value)
    }

    pub fn remove(&self, v: &mut Value, pointer: &str) -> Result<()> {
        JsonTool::new().remove(v, pointer)
    }

    pub fn patch(&self, v: &mut Value, p: &Patch) -> Result<()> {
        JsonTool::new().patch(v, p)
    }
}
