use serde_json::{Map, Value};
use teaql_tool_core::{Result, TeaQLToolError};

/// Tree structure utility wrapper
pub struct TreeTool;

impl TreeTool {
    pub fn new() -> Self {
        Self
    }

    /// Build a nested tree from a flat JSON array of objects.
    /// `flat_array`: A JSON array containing flat objects.
    /// `id_field`: The property name acting as the unique identifier.
    /// `parent_id_field`: The property name acting as the parent identifier.
    /// `children_field`: The property name where children will be inserted.
    /// `root_parent_id`: The parent ID value that identifies root elements (e.g. `0` or `null`).
    pub fn build(
        &self,
        flat_array: Value,
        id_field: &str,
        parent_id_field: &str,
        children_field: &str,
        root_parent_id: Value,
    ) -> Result<Value> {
        let arr = flat_array.as_array().ok_or_else(|| {
            TeaQLToolError::InvalidArgument("flat_array must be a JSON array".to_string())
        })?;

        // Extract items as Map
        let mut items: Vec<Map<String, Value>> = Vec::new();
        for item in arr {
            if let Some(obj) = item.as_object() {
                items.push(obj.clone());
            } else {
                return Err(TeaQLToolError::InvalidArgument(
                    "All elements in flat_array must be objects".to_string(),
                ));
            }
        }

        let mut forest = Vec::new();

        // Very basic O(N^2) builder for simplicity and avoiding borrow checker self-referencing.
        // For enterprise sizes usually N is small (menus, org charts < 1000 items).
        // A recursive approach or hashmap based approach is possible but requires Rc<RefCell> in Rust.
        
        // We will do a multi-pass to build the tree.
        let mut all_nodes = items.clone();
        
        fn build_children(
            parent_id: &Value,
            nodes: &[Map<String, Value>],
            id_f: &str,
            pid_f: &str,
            child_f: &str,
        ) -> Vec<Value> {
            let mut children = Vec::new();
            for node in nodes {
                if let Some(pid) = node.get(pid_f) {
                    if pid == parent_id {
                        let mut cloned_node = node.clone();
                        let current_id = node.get(id_f).unwrap_or(&Value::Null);
                        let sub_children = build_children(current_id, nodes, id_f, pid_f, child_f);
                        if !sub_children.is_empty() {
                            cloned_node.insert(child_f.to_string(), Value::Array(sub_children));
                        }
                        children.push(Value::Object(cloned_node));
                    }
                }
            }
            children
        }

        forest = build_children(&root_parent_id, &all_nodes, id_field, parent_id_field, children_field);
        Ok(Value::Array(forest))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_tree_build() {
        let tool = TreeTool::new();
        let flat = json!([
            {"id": 1, "pid": 0, "name": "Root"},
            {"id": 2, "pid": 1, "name": "Child 1"},
            {"id": 3, "pid": 1, "name": "Child 2"},
            {"id": 4, "pid": 2, "name": "Grandchild"}
        ]);

        let tree = tool.build(flat, "id", "pid", "children", json!(0)).unwrap();
        let arr = tree.as_array().unwrap();
        assert_eq!(arr.len(), 1);
        let root = arr[0].as_object().unwrap();
        assert_eq!(root.get("name").unwrap().as_str().unwrap(), "Root");
        
        let children = root.get("children").unwrap().as_array().unwrap();
        assert_eq!(children.len(), 2);
    }
}
