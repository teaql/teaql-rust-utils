use teaql_tool_core::{Result, TeaQLToolError};
use scraper::{Html, Selector};

#[derive(Debug, Clone)]
pub struct HtmlTool;

impl HtmlTool {
    pub fn new() -> Self { Self }

    pub fn select_text(&self, html_str: &str, selector_str: &str) -> Result<Vec<String>> {
        let document = Html::parse_document(html_str);
        let selector = Selector::parse(selector_str).map_err(|e| TeaQLToolError::ParseError(format!("{:?}", e)))?;
        
        let mut results = Vec::new();
        for element in document.select(&selector) {
            results.push(element.text().collect::<Vec<_>>().join(" "));
        }
        Ok(results)
    }

    pub fn select_attr(&self, html_str: &str, selector_str: &str, attr: &str) -> Result<Vec<String>> {
        let document = Html::parse_document(html_str);
        let selector = Selector::parse(selector_str).map_err(|e| TeaQLToolError::ParseError(format!("{:?}", e)))?;
        
        let mut results = Vec::new();
        for element in document.select(&selector) {
            if let Some(val) = element.value().attr(attr) {
                results.push(val.to_string());
            }
        }
        Ok(results)
    }
}

impl Default for HtmlTool {
    fn default() -> Self { Self::new() }
}
