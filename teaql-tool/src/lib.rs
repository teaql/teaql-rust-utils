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
    pub fn color() -> teaql_tool_std::color::ColorTool {
        teaql_tool_std::color::ColorTool::new()
    }

    #[cfg(feature = "std")]
    pub fn daterange() -> teaql_tool_std::daterange::DateRangeTool {
        teaql_tool_std::daterange::DateRangeTool::new()
    }

    #[cfg(feature = "std")]
    pub fn diff() -> teaql_tool_std::diff::DiffTool {
        teaql_tool_std::diff::DiffTool::new()
    }

    #[cfg(feature = "std")]
    pub fn i18n() -> teaql_tool_std::i18n::I18nTool {
        teaql_tool_std::i18n::I18nTool::new()
    }

    #[cfg(feature = "std")]
    pub fn list() -> teaql_tool_std::list::ListTool {
        teaql_tool_std::list::ListTool::new()
    }

    #[cfg(feature = "std")]
    pub fn map() -> teaql_tool_std::map::MapTool {
        teaql_tool_std::map::MapTool::new()
    }

    #[cfg(feature = "std")]
    pub fn system() -> teaql_tool_std::system::SystemTool {
        teaql_tool_std::system::SystemTool::new()
    }

    #[cfg(feature = "std")]
    pub fn unit() -> teaql_tool_std::unit::UnitTool {
        teaql_tool_std::unit::UnitTool::new()
    }

    #[cfg(feature = "std")]
    pub fn url() -> teaql_tool_std::url::UrlTool {
        teaql_tool_std::url::UrlTool::new()
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
    pub fn address() -> teaql_tool_extra::address::AddressTool {
        teaql_tool_extra::address::AddressTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn barcode() -> teaql_tool_extra::barcode::BarcodeTool {
        teaql_tool_extra::barcode::BarcodeTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn cache() -> teaql_tool_extra::cache::CacheTool {
        teaql_tool_extra::cache::CacheTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn config() -> teaql_tool_extra::config::ConfigTool {
        teaql_tool_extra::config::ConfigTool::new()
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
    pub fn email() -> teaql_tool_extra::email::EmailTool {
        teaql_tool_extra::email::EmailTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn excel() -> teaql_tool_extra::excel::ExcelTool {
        teaql_tool_extra::excel::ExcelTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn geo() -> teaql_tool_extra::geo::GeoTool {
        teaql_tool_extra::geo::GeoTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn http() -> teaql_tool_extra::http::HttpTool {
        teaql_tool_extra::http::HttpTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn image() -> teaql_tool_extra::image::ImageTool {
        teaql_tool_extra::image::ImageTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn jwt() -> teaql_tool_extra::jwt::JwtTool {
        teaql_tool_extra::jwt::JwtTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn phone() -> teaql_tool_extra::phone::PhoneTool {
        teaql_tool_extra::phone::PhoneTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn qrcode() -> teaql_tool_extra::qrcode::QrcodeTool {
        teaql_tool_extra::qrcode::QrcodeTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn random() -> teaql_tool_extra::random::RandomTool {
        teaql_tool_extra::random::RandomTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn template() -> teaql_tool_extra::template::TemplateTool {
        teaql_tool_extra::template::TemplateTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn proxy() -> teaql_tool_extra::proxy::ProxyTool {
        teaql_tool_extra::proxy::ProxyTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn server() -> teaql_tool_extra::server::ServerTool {
        teaql_tool_extra::server::ServerTool::new()
    }

    #[cfg(feature = "extra")]
    pub fn watcher() -> teaql_tool_extra::watcher::WatcherTool {
        teaql_tool_extra::watcher::WatcherTool::new()
    }
}
