use teaql_tool_core::{Result, TeaQLToolError};
use std::fs::File;
use zip::write::SimpleFileOptions;

#[derive(Debug, Clone)]
pub struct ArchiveTool;

impl ArchiveTool {
    pub fn new() -> Self { Self }

    pub fn zip_dir(&self, src_dir: &str, dst_file: &str) -> Result<()> {
        let file = File::create(dst_file).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        let mut zip = zip::ZipWriter::new(file);
        let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        
        let walkdir = walkdir::WalkDir::new(src_dir);
        let it = walkdir.into_iter();
        
        for entry in it.filter_map(|e| e.ok()) {
            let path = entry.path();
            let name = path.strip_prefix(std::path::Path::new(src_dir)).unwrap();
            let name_str = name.to_str().unwrap().replace("\\", "/");
            
            if path.is_file() {
                zip.start_file(name_str, options).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
                let mut f = File::open(path).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
                std::io::copy(&mut f, &mut zip).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
            } else if !name.as_os_str().is_empty() {
                zip.add_directory(name_str, options).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
            }
        }
        zip.finish().map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        Ok(())
    }

    pub fn unzip(&self, src_file: &str, dst_dir: &str) -> Result<()> {
        let file = File::open(src_file).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        let mut archive = zip::ZipArchive::new(file).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
            let outpath = match file.enclosed_name() {
                Some(path) => std::path::Path::new(dst_dir).join(path),
                None => continue,
            };
            
            if (*file.name()).ends_with('/') {
                std::fs::create_dir_all(&outpath).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(&p).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
                    }
                }
                let mut outfile = File::create(&outpath).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
                std::io::copy(&mut file, &mut outfile).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
            }
        }
        Ok(())
    }
}

impl Default for ArchiveTool {
    fn default() -> Self { Self::new() }
}
