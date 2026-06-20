use crate::macros::define_context_facade;

use teaql_tool_std::desensitize::DesensitizeTool;

define_context_facade!("std", desensitize, ContextDesensitizeExt, ContextDesensitizeFacade);

#[cfg(feature = "std")]
impl<'a> ContextDesensitizeFacade<'a> {
    /// Desensitize Chinese ID Card (e.g. 110105199001011234 -> 110105********1234)
    pub fn chinese_id_card(&self, id_card: impl AsRef<str>) -> String {
        DesensitizeTool::new().chinese_id_card(id_card)
    }

    /// Desensitize Chinese Phone (e.g. 13812345678 -> 138****5678)
    pub fn chinese_phone(&self, phone: impl AsRef<str>) -> String {
        DesensitizeTool::new().chinese_phone(phone)
    }

    /// Desensitize Chinese Name (e.g. 张三 -> 张*, 王大拿 -> 王*拿)
    pub fn chinese_name(&self, name: impl AsRef<str>) -> String {
        DesensitizeTool::new().chinese_name(name)
    }

    /// Desensitize US Social Security Number (SSN) (e.g. 123-45-6789 -> ***-**-6789)
    pub fn us_ssn(&self, ssn: impl AsRef<str>) -> String {
        DesensitizeTool::new().us_ssn(ssn)
    }

    /// Desensitize US Phone Number (e.g. +1-555-123-4567 -> +1-***-***-4567 or 555-123-4567 -> ***-***-4567)
    pub fn us_phone(&self, phone: impl AsRef<str>) -> String {
        DesensitizeTool::new().us_phone(phone)
    }

    /// Desensitize Credit/Debit Card (e.g. 1234 5678 1234 5678 -> **** **** **** 5678)
    pub fn credit_card(&self, card: impl AsRef<str>) -> String {
        DesensitizeTool::new().credit_card(card)
    }

    /// Desensitize Email (e.g. user@example.com -> u***@example.com)
    pub fn email(&self, email: impl AsRef<str>) -> String {
        DesensitizeTool::new().email(email)
    }

    /// Mask password to fixed asterisks
    pub fn password(&self, pwd: impl AsRef<str>) -> String {
        DesensitizeTool::new().password(pwd)
    }
}
