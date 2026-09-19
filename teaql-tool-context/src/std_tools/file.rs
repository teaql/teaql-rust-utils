use crate::macros::*;

use std::path::{Path, PathBuf};
use teaql_tool_core::{MustAuditAs, Result};
use teaql_tool_std::file::FileTool;

define_context_facade!("std", file, ContextFileExt, ContextFileFacade);

#[cfg(feature = "std")]
impl<'a> ContextFileFacade<'a> {
    /// Read the entire contents of a file into a string
    pub fn read_string<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> teaql_tool_core::Result<teaql_tool_core::MustPurpose<String>> {
        FileTool
            .read_string(path)
            .map(teaql_tool_core::MustPurpose::new)
    }

    /// Read the entire contents of a file into a bytes vector
    pub fn read_bytes<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> teaql_tool_core::Result<teaql_tool_core::MustPurpose<Vec<u8>>> {
        FileTool
            .read_bytes(path)
            .map(teaql_tool_core::MustPurpose::new)
    }

    /// Write a string to a file (creates or overwrites)
    pub fn write_string<P: AsRef<Path>, C: AsRef<[u8]>>(
        &self,
        path: P,
        content: C,
    ) -> MustAuditAs<Result<()>> {
        let path = path.as_ref().to_path_buf();
        let content = content.as_ref().to_vec();
        MustAuditAs::new(move |_desc| FileTool.write_string(path, content))
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
    pub fn mkdir<P: AsRef<Path>>(&self, path: P) -> MustAuditAs<Result<()>> {
        let path = path.as_ref().to_path_buf();
        MustAuditAs::new(move |_desc| FileTool.mkdir(path))
    }

    /// Recursively create a directory and all of its parent components if they are missing
    pub fn mkdir_all<P: AsRef<Path>>(&self, path: P) -> MustAuditAs<Result<()>> {
        let path = path.as_ref().to_path_buf();
        MustAuditAs::new(move |_desc| FileTool.mkdir_all(path))
    }

    /// Delete a file
    pub fn delete_file<P: AsRef<Path>>(&self, path: P) -> MustAuditAs<Result<()>> {
        let path = path.as_ref().to_path_buf();
        MustAuditAs::new(move |_desc| FileTool.delete_file(path))
    }

    /// Delete an empty directory
    pub fn delete_dir<P: AsRef<Path>>(&self, path: P) -> MustAuditAs<Result<()>> {
        let path = path.as_ref().to_path_buf();
        MustAuditAs::new(move |_desc| FileTool.delete_dir(path))
    }

    /// Recursively delete a directory and all of its contents
    pub fn delete_recursive<P: AsRef<Path>>(&self, path: P) -> MustAuditAs<Result<()>> {
        let path = path.as_ref().to_path_buf();
        MustAuditAs::new(move |_desc| FileTool.delete_recursive(path))
    }

    /// Copy a file
    pub fn copy<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> MustAuditAs<Result<u64>> {
        let from = from.as_ref().to_path_buf();
        let to = to.as_ref().to_path_buf();
        MustAuditAs::new(move |_desc| FileTool.copy(from, to))
    }

    /// Rename or move a file/directory
    pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        from: P,
        to: Q,
    ) -> MustAuditAs<Result<()>> {
        let from = from.as_ref().to_path_buf();
        let to = to.as_ref().to_path_buf();
        MustAuditAs::new(move |_desc| FileTool.rename(from, to))
    }

    /// List only files in a directory (non-recursive)
    pub fn list_files<P: AsRef<Path>>(
        &self,
        dir: P,
    ) -> teaql_tool_core::Result<teaql_tool_core::MustPurpose<Vec<PathBuf>>> {
        FileTool
            .list_files(dir)
            .map(teaql_tool_core::MustPurpose::new)
    }

    /// List only directories in a directory (non-recursive)
    pub fn list_dirs<P: AsRef<Path>>(
        &self,
        dir: P,
    ) -> teaql_tool_core::Result<teaql_tool_core::MustPurpose<Vec<PathBuf>>> {
        FileTool
            .list_dirs(dir)
            .map(teaql_tool_core::MustPurpose::new)
    }
}
