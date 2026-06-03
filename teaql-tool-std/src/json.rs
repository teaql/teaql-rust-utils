use json_patch::{Patch, diff, patch};
use serde_json::{Value, from_str};
use teaql_tool_core::{MustComment, Result, TeaQLToolError};

pub struct JsonTool;

impl JsonTool {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, s: &str) -> Result<MustComment<Value>> {
        from_str(s)
            .map(MustComment::new)
            .map_err(|e| TeaQLToolError::ParseError(e.to_string()))
    }

    pub fn stringify(&self, v: &Value) -> Result<MustComment<String>> {
        serde_json::to_string(v)
            .map(MustComment::new)
            .map_err(|e| TeaQLToolError::ParseError(e.to_string()))
    }

    pub fn stringify_pretty(&self, v: &Value) -> Result<MustComment<String>> {
        serde_json::to_string_pretty(v)
            .map(MustComment::new)
            .map_err(|e| TeaQLToolError::ParseError(e.to_string()))
    }

    pub fn get<'a>(&self, v: &'a Value, pointer: &str) -> MustComment<Option<&'a Value>> {
        MustComment::new(v.pointer(pointer))
    }

    pub fn set(&self, v: &mut Value, pointer: &str, value: Value) -> Result<MustComment<()>> {
        let ptr = v.pointer_mut(pointer);
        if let Some(target) = ptr {
            *target = value;
            Ok(MustComment::new(()))
        } else {
            // Very naive way, json_patch provides add but pointer_mut requires path to exist
            let p = format!(
                r#"[{{"op": "add", "path": "{}", "value": {}}}]"#,
                pointer, value
            );
            let patch_obj = serde_json::from_str::<Patch>(&p).unwrap();
            self.patch(v, &patch_obj)
        }
    }

    pub fn remove(&self, v: &mut Value, pointer: &str) -> Result<MustComment<()>> {
        let p = format!(r#"[{{"op": "remove", "path": "{}"}}]"#, pointer);
        let patch_obj = serde_json::from_str::<Patch>(&p).unwrap();
        self.patch(v, &patch_obj)
    }

    pub fn has(&self, v: &Value, pointer: &str) -> MustComment<bool> {
        MustComment::new(v.pointer(pointer).is_some())
    }

    pub fn merge(&self, a: &Value, b: &Value) -> MustComment<Value> {
        let mut result = a.clone();
        if let (Value::Object(r), Value::Object(b_obj)) = (&mut result, b) {
            for (k, v) in b_obj {
                r.insert(k.clone(), v.clone());
            }
        }
        MustComment::new(result)
    }

    pub fn diff(&self, a: &Value, b: &Value) -> MustComment<Patch> {
        MustComment::new(diff(a, b))
    }

    pub fn patch(&self, v: &mut Value, p: &Patch) -> Result<MustComment<()>> {
        patch(v, p)
            .map(|_| MustComment::new(()))
            .map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))
    }
}

impl Default for JsonTool {
    fn default() -> Self {
        Self::new()
    }
}
