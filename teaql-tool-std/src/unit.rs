#[derive(Debug, Clone)]
pub struct UnitTool;

impl UnitTool {
    pub fn new() -> Self { Self }

    pub fn bytes_to_kb(&self, bytes: u64) -> f64 { bytes as f64 / 1024.0 }
    pub fn bytes_to_mb(&self, bytes: u64) -> f64 { bytes as f64 / 1024.0 / 1024.0 }
    pub fn bytes_to_gb(&self, bytes: u64) -> f64 { bytes as f64 / 1024.0 / 1024.0 / 1024.0 }
    
    pub fn c_to_f(&self, c: f64) -> f64 { c * 9.0 / 5.0 + 32.0 }
    pub fn f_to_c(&self, f: f64) -> f64 { (f - 32.0) * 5.0 / 9.0 }
}

impl Default for UnitTool {
    fn default() -> Self { Self::new() }
}
