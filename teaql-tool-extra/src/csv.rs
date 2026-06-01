use csv::{Reader, Writer};
use teaql_tool_core::{Result, TeaQLToolError};

pub struct CsvTool;

impl CsvTool {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, data: &str) -> Result<Vec<Vec<String>>> {
        let mut rdr = Reader::from_reader(data.as_bytes());
        let mut records = Vec::new();
        for result in rdr.records() {
            let record = result.map_err(|e| TeaQLToolError::ParseError(e.to_string()))?;
            records.push(record.iter().map(|s| s.to_string()).collect());
        }
        Ok(records)
    }

    pub fn generate(&self, records: &[Vec<String>]) -> Result<String> {
        let mut wtr = Writer::from_writer(vec![]);
        for record in records {
            wtr.write_record(record).map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        }
        let data = wtr.into_inner().map_err(|e| TeaQLToolError::ExecutionError(e.to_string()))?;
        String::from_utf8(data).map_err(|e| TeaQLToolError::ParseError(e.to_string()))
    }
}

impl Default for CsvTool {
    fn default() -> Self {
        Self::new()
    }
}
