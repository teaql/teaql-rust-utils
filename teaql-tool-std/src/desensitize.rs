pub struct DesensitizeTool;

impl DesensitizeTool {
    pub fn new() -> Self {
        Self
    }

    // ================== Chinese Formats ==================

    /// Desensitize Chinese ID Card (e.g. 110105199001011234 -> 110105********1234)
    pub fn chinese_id_card(&self, id_card: impl AsRef<str>) -> String {
        let s = id_card.as_ref();
        if s.len() == 18 {
            format!("{}********{}", &s[0..6], &s[14..18])
        } else if s.len() == 15 {
            format!("{}******{}", &s[0..6], &s[12..15])
        } else {
            s.to_string()
        }
    }

    /// Desensitize Chinese Phone (e.g. 13812345678 -> 138****5678)
    pub fn chinese_phone(&self, phone: impl AsRef<str>) -> String {
        let s = phone.as_ref();
        if s.len() == 11 {
            format!("{}****{}", &s[0..3], &s[7..11])
        } else {
            s.to_string()
        }
    }

    /// Desensitize Chinese Name (e.g. 张三 -> 张*, 王大拿 -> 王*拿)
    pub fn chinese_name(&self, name: impl AsRef<str>) -> String {
        let s = name.as_ref();
        let chars: Vec<char> = s.chars().collect();
        match chars.len() {
            0 => String::new(),
            1 => s.to_string(),
            2 => format!("{}*", chars[0]),
            _ => format!("{}*{}", chars[0], chars[chars.len() - 1]),
        }
    }

    // ================== US / International Formats ==================

    /// Desensitize US Social Security Number (SSN) (e.g. 123-45-6789 -> ***-**-6789)
    pub fn us_ssn(&self, ssn: impl AsRef<str>) -> String {
        let s = ssn.as_ref();
        if s.len() == 11 && s.chars().nth(3) == Some('-') && s.chars().nth(6) == Some('-') {
            format!("***-**-{}", &s[7..11])
        } else if s.len() == 9 {
            format!("*****{}", &s[5..9])
        } else {
            s.to_string()
        }
    }

    /// Desensitize US Phone Number (e.g. +1-555-123-4567 -> +1-***-***-4567 or 555-123-4567 -> ***-***-4567)
    pub fn us_phone(&self, phone: impl AsRef<str>) -> String {
        let s = phone.as_ref();
        // A simple approach: mask all digits except the last 4
        let mut result = String::with_capacity(s.len());
        let digit_count = s.chars().filter(|c| c.is_ascii_digit()).count();
        let mut digits_seen = 0;
        
        for c in s.chars() {
            if c.is_ascii_digit() {
                digits_seen += 1;
                if digits_seen <= digit_count.saturating_sub(4) {
                    result.push('*');
                } else {
                    result.push(c);
                }
            } else {
                result.push(c);
            }
        }
        result
    }

    /// Desensitize Credit/Debit Card (e.g. 1234 5678 1234 5678 -> **** **** **** 5678)
    pub fn credit_card(&self, card: impl AsRef<str>) -> String {
        let s = card.as_ref();
        let mut result = String::with_capacity(s.len());
        let digit_count = s.chars().filter(|c| c.is_ascii_digit()).count();
        let mut digits_seen = 0;

        for c in s.chars() {
            if c.is_ascii_digit() {
                digits_seen += 1;
                if digits_seen <= digit_count.saturating_sub(4) {
                    result.push('*');
                } else {
                    result.push(c);
                }
            } else {
                result.push(c);
            }
        }
        result
    }

    // ================== Generic ==================

    /// Desensitize Email (e.g. user@example.com -> u***@example.com)
    pub fn email(&self, email: impl AsRef<str>) -> String {
        let s = email.as_ref();
        if let Some(idx) = s.find('@') {
            let prefix = &s[..idx];
            let domain = &s[idx..];
            if prefix.len() <= 1 {
                format!("*{}", domain)
            } else {
                format!("{}***{}", &prefix[..1], domain)
            }
        } else {
            s.to_string()
        }
    }

    /// Mask password to fixed asterisks
    pub fn password(&self, _pwd: impl AsRef<str>) -> String {
        "******".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_desensitize() {
        let t = DesensitizeTool::new();
        assert_eq!(t.chinese_phone("13812345678"), "138****5678");
        assert_eq!(t.chinese_id_card("110105199001011234"), "110105********1234");
        assert_eq!(t.chinese_name("张三"), "张*");
        assert_eq!(t.chinese_name("王大拿"), "王*拿");
        assert_eq!(t.us_ssn("123-45-6789"), "***-**-6789");
        assert_eq!(t.us_phone("+1-555-123-4567"), "+*-***-***-4567");
        assert_eq!(t.credit_card("1234-5678-1234-5678"), "****-****-****-5678");
        assert_eq!(t.email("john.doe@example.com"), "j***@example.com");
    }
}
