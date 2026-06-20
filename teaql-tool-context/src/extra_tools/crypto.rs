use crate::macros::*;

use teaql_tool_extra::crypto::CryptoTool;
use teaql_tool_core::Result;

define_context_facade!("extra", crypto, ContextCryptoExt, ContextCryptoFacade);

#[cfg(feature = "extra")]
impl<'a> ContextCryptoFacade<'a> {
    delegate_comment! { CryptoTool::new(),
        fn generate_key(&self) -> Vec<u8>
    }
    delegate_res_comment! { CryptoTool::new(),
        fn encrypt(&self, data: &[u8], key: &[u8]) -> Vec<u8>;
        fn decrypt(&self, encrypted_data: &[u8], key: &[u8]) -> Vec<u8>
    }
}
