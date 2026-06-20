use crate::macros::*;

use teaql_tool_std::file::FileTool;
use std::path::{Path, PathBuf};
use teaql_tool_core::Result;

define_context_facade!("std", file, ContextFileExt, ContextFileFacade);

#[cfg(feature = "std")]
impl<'a> ContextFileFacade<'a> {
    /// Read the entire contents of a file into a string
    pub fn read_string<P: AsRef<Path>>(&self, path: P) -> teaql_tool_core::Result<teaql_tool_core::MustPurpose<String>> {
        FileTool.read_string(path).map(teaql_tool_core::MustPurpose::new)
    }

    /// Read the entire contents of a file into a bytes vector
    pub fn read_bytes<P: AsRef<Path>>(&self, path: P) -> teaql_tool_core::Result<teaql_tool_core::MustPurpose<Vec<u8>>> {
        FileTool.read_bytes(path).map(teaql_tool_core::MustPurpose::new)
    }

    /// Write a string to a file (creates or overwrites)
    pub fn write_string<P: AsRef<Path>, C: AsRef<[u8]>>(&self, path: P, content: C) -> teaql_tool_core::Result<teaql_tool_core::MustAuditAs<()>> {
        FileTool.write_string(path, content).map(teaql_tool_core::MustAuditAs::new)
    }

    /// Check if a path exists
    pub fn exists<P: AsRef<Path>>(&self, path: P) -> teaql_tool_core::MustPurpose<bool> {
        teaql_tool_core::MustPurpose::new(FileTool.exists(path))
    }

    /// Check if a path is a file
    pub fn is_file<P: AsRef<Path>>(&self, path: P) -> teaql_tool_core::MustPurpose<bool> {
        teaql_tool_core::MustPurpose::new(FileTool.is_file(path))
    }

    /// Check if a path is a directory
    pub fn is_dir<P: AsRef<Path>>(&self, path: P) -> teaql_tool_core::MustPurpose<bool> {
        teaql_tool_core::MustPurpose::new(FileTool.is_dir(path))
    }

    /// Create a single directory (fails if parent doesn't exist)
    pub fn mkdir<P: AsRef<Path>>(&self, path: P) -> teaql_tool_core::Result<teaql_tool_core::MustAuditAs<()>> {
        FileTool.mkdir(path).map(teaql_tool_core::MustAuditAs::new)
    }

    /// Recursively create a directory and all of its parent components if they are missing
    pub fn mkdir_all<P: AsRef<Path>>(&self, path: P) -> teaql_tool_core::Result<teaql_tool_core::MustAuditAs<()>> {
        FileTool.mkdir_all(path).map(teaql_tool_core::MustAuditAs::new)
    }

    /// Delete a file
    pub fn delete_file<P: AsRef<Path>>(&self, path: P) -> teaql_tool_core::Result<teaql_tool_core::MustAuditAs<()>> {
        FileTool.delete_file(path).map(teaql_tool_core::MustAuditAs::new)
    }

    /// Delete an empty directory
    pub fn delete_dir<P: AsRef<Path>>(&self, path: P) -> teaql_tool_core::Result<teaql_tool_core::MustAuditAs<()>> {
        FileTool.delete_dir(path).map(teaql_tool_core::MustAuditAs::new)
    }

    /// Recursively delete a directory and all of its contents
    pub fn delete_recursive<P: AsRef<Path>>(&self, path: P) -> teaql_tool_core::Result<teaql_tool_core::MustAuditAs<()>> {
        FileTool.delete_recursive(path).map(teaql_tool_core::MustAuditAs::new)
    }

    /// Copy a file
    pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> teaql_tool_core::Result<teaql_tool_core::MustAuditAs<u64>> {
        FileTool.copy(from, to).map(teaql_tool_core::MustAuditAs::new)
    }

    /// Rename or move a file/directory
    pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> teaql_tool_core::Result<teaql_tool_core::MustAuditAs<()>> {
        FileTool.rename(from, to).map(teaql_tool_core::MustAuditAs::new)
    }

    /// List only files in a directory (non-recursive)
    pub fn list_files<P: AsRef<Path>>(&self, dir: P) -> teaql_tool_core::Result<teaql_tool_core::MustPurpose<Vec<PathBuf>>> {
        FileTool.list_files(dir).map(teaql_tool_core::MustPurpose::new)
    }

    /// List only directories in a directory (non-recursive)
    pub fn list_dirs<P: AsRef<Path>>(&self, dir: P) -> teaql_tool_core::Result<teaql_tool_core::MustPurpose<Vec<PathBuf>>> {
        FileTool.list_dirs(dir).map(teaql_tool_core::MustPurpose::new)
    }
}
