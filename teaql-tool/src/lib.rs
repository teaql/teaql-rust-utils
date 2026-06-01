pub use teaql_tool_core::{Result, TeaQLToolError};

pub struct T;

impl T {
    #[cfg(feature = "std")]
    pub fn time() -> teaql_tool_std::time::TimeTool {
        teaql_tool_std::time::TimeTool::new()
    }

    #[cfg(feature = "std")]
    pub fn id() -> teaql_tool_std::id::IdTool {
        teaql_tool_std::id::IdTool::new()
    }

    #[cfg(feature = "std")]
    pub fn text() -> teaql_tool_std::text::TextTool {
        teaql_tool_std::text::TextTool::new()
    }

    #[cfg(feature = "std")]
    pub fn json() -> teaql_tool_std::json::JsonTool {
        teaql_tool_std::json::JsonTool::new()
    }

    #[cfg(feature = "std")]
    pub fn regex() -> teaql_tool_std::regex::RegexTool {
        teaql_tool_std::regex::RegexTool::new()
    }

    #[cfg(feature = "std")]
    pub fn decimal() -> teaql_tool_std::decimal::DecimalTool {
        teaql_tool_std::decimal::DecimalTool::new()
    }

    #[cfg(feature = "std")]
    pub fn money() -> teaql_tool_std::money::MoneyTool {
        teaql_tool_std::money::MoneyTool::new()
    }

    #[cfg(feature = "std")]
    pub fn codec() -> teaql_tool_std::codec::CodecTool {
        teaql_tool_std::codec::CodecTool::new()
    }

    #[cfg(feature = "std")]
    pub fn hash() -> teaql_tool_std::hash::HashTool {
        teaql_tool_std::hash::HashTool::new()
    }

    #[cfg(feature = "std")]
    pub fn system() -> teaql_tool_std::system::SystemTool {
        teaql_tool_std::system::SystemTool::new()
    }

    #[cfg(feature = "std")]
    pub fn filter() -> teaql_tool_std::filter::FilterTool {
        teaql_tool_std::filter::FilterTool::new()
    }

    #[cfg(feature = "std")]
    pub fn validate() -> teaql_tool_std::validate::ValidateTool {
        teaql_tool_std::validate::ValidateTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn random() -> teaql_tool_extra::random::RandomTool {
        teaql_tool_extra::random::RandomTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn crypto() -> teaql_tool_extra::crypto::CryptoTool {
        teaql_tool_extra::crypto::CryptoTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn csv() -> teaql_tool_extra::csv::CsvTool {
        teaql_tool_extra::csv::CsvTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn http() -> teaql_tool_extra::http::HttpTool {
        teaql_tool_extra::http::HttpTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn jwt() -> teaql_tool_extra::jwt::JwtTool {
        teaql_tool_extra::jwt::JwtTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn config() -> teaql_tool_extra::config::ConfigTool {
        teaql_tool_extra::config::ConfigTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn cache() -> teaql_tool_extra::cache::CacheTool {
        teaql_tool_extra::cache::CacheTool::new()
    }
}
