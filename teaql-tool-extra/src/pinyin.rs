#[derive(Debug, Clone)]
pub struct PinyinTool;

impl PinyinTool {
    pub fn new() -> Self { Self }

    pub fn get_pinyin(&self, text: &str) -> String {
        let mut res = Vec::new();
        for p in pinyin::pinyin(text, &pinyin::Args::new()) {
            if let Some(py) = p.get(0) {
                res.push(py.to_string());
            }
        }
        res.join(" ")
    }
}

impl Default for PinyinTool {
    fn default() -> Self { Self::new() }
}
