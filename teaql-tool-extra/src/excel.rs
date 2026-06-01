use teaql_tool_core::{Result, TeaQLToolError};
use rust_xlsxwriter::Workbook;
use calamine::{Reader, open_workbook_auto};

#[derive(Debug, Clone)]
pub struct ExcelTool;

impl ExcelTool {
    pub fn new() -> Self { Self }

    pub fn write_simple(&self, path: &str, data: &[Vec<String>]) -> Result<()> {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();
        
        for (row, r_data) in data.iter().enumerate() {
            for (col, val) in r_data.iter().enumerate() {
                worksheet.write_string(row as u32, col as u16, val).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
            }
        }
        
        workbook.save(path).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        Ok(())
    }

    pub fn read_simple(&self, path: &str) -> Result<Vec<Vec<String>>> {
        let mut workbook = open_workbook_auto(path).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        let sheet_name = workbook.sheet_names().get(0).cloned().unwrap_or_default();
        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            let mut result = Vec::new();
            for row in range.rows() {
                let row_data: Vec<String> = row.iter().map(|c| c.to_string()).collect();
                result.push(row_data);
            }
            Ok(result)
        } else {
            Ok(Vec::new())
        }
    }
}

impl Default for ExcelTool {
    fn default() -> Self { Self::new() }
}
