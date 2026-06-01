use std::fs;
use std::path::{Path, PathBuf};
use teaql_tool_core::{Result, TeaQLToolError};

/// File utility wrapper
pub struct FileTool;

impl FileTool {
    /// Read the entire contents of a file into a string
    pub fn read_string<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        fs::read_to_string(path.as_ref())
            .map_err(|e| TeaQLToolError::ExecutionError(format!("Failed to read file: {}", e)))
    }

    /// Read the entire contents of a file into a bytes vector
    pub fn read_bytes<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>> {
        fs::read(path.as_ref())
            .map_err(|e| TeaQLToolError::ExecutionError(format!("Failed to read bytes: {}", e)))
    }

    /// Write a string to a file (creates or overwrites)
    pub fn write_string<P: AsRef<Path>, C: AsRef<[u8]>>(&self, path: P, content: C) -> Result<()> {
        fs::write(path.as_ref(), content)
            .map_err(|e| TeaQLToolError::ExecutionError(format!("Failed to write file: {}", e)))
    }

    /// Check if a path exists
    pub fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().exists()
    }

    /// Check if a path is a file
    pub fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().is_file()
    }

    /// Check if a path is a directory
    pub fn is_dir<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().is_dir()
    }

    /// Create a single directory (fails if parent doesn't exist)
    pub fn mkdir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::create_dir(path.as_ref())
            .map_err(|e| TeaQLToolError::ExecutionError(format!("Failed to create directory: {}", e)))
    }

    /// Recursively create a directory and all of its parent components if they are missing
    pub fn mkdir_all<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::create_dir_all(path.as_ref())
            .map_err(|e| TeaQLToolError::ExecutionError(format!("Failed to create directories recursively: {}", e)))
    }

    /// Delete a file
    pub fn delete_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::remove_file(path.as_ref())
            .map_err(|e| TeaQLToolError::ExecutionError(format!("Failed to delete file: {}", e)))
    }

    /// Delete an empty directory
    pub fn delete_dir<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::remove_dir(path.as_ref())
            .map_err(|e| TeaQLToolError::ExecutionError(format!("Failed to delete directory: {}", e)))
    }

    /// Recursively delete a directory and all of its contents
    pub fn delete_recursive<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        fs::remove_dir_all(path.as_ref())
            .map_err(|e| TeaQLToolError::ExecutionError(format!("Failed to recursively delete directory: {}", e)))
    }

    /// Copy a file
    pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<u64> {
        fs::copy(from.as_ref(), to.as_ref())
            .map_err(|e| TeaQLToolError::ExecutionError(format!("Failed to copy file: {}", e)))
    }

    /// Rename or move a file/directory
    pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<()> {
        fs::rename(from.as_ref(), to.as_ref())
            .map_err(|e| TeaQLToolError::ExecutionError(format!("Failed to rename/move: {}", e)))
    }

    /// List only files in a directory (non-recursive)
    pub fn list_files<P: AsRef<Path>>(&self, dir: P) -> Result<Vec<PathBuf>> {
        self.list_entries(dir, true, false)
    }

    /// List only directories in a directory (non-recursive)
    pub fn list_dirs<P: AsRef<Path>>(&self, dir: P) -> Result<Vec<PathBuf>> {
        self.list_entries(dir, false, true)
    }

    /// Internal helper to list entries
    fn list_entries<P: AsRef<Path>>(&self, dir: P, include_files: bool, include_dirs: bool) -> Result<Vec<PathBuf>> {
        let mut result = Vec::new();
        let entries = fs::read_dir(dir.as_ref())
            .map_err(|e| TeaQLToolError::ExecutionError(format!("Failed to read directory: {}", e)))?;
            
        for entry in entries {
            let entry = entry.map_err(|e| TeaQLToolError::ExecutionError(format!("Error reading directory entry: {}", e)))?;
            let path = entry.path();
            if path.is_file() && include_files {
                result.push(path);
            } else if path.is_dir() && include_dirs {
                result.push(path);
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_file_operations() {
        let tool = FileTool;
        let mut tmp_path = env::temp_dir();
        tmp_path.push("teaql_test_file.txt");
        
        // Write
        tool.write_string(&tmp_path, "hello teaql").unwrap();
        assert!(tool.exists(&tmp_path));
        assert!(tool.is_file(&tmp_path));
        
        // Read
        let content = tool.read_string(&tmp_path).unwrap();
        assert_eq!(content, "hello teaql");
        
        // Delete
        tool.delete_file(&tmp_path).unwrap();
        assert!(!tool.exists(&tmp_path));
    }
}
