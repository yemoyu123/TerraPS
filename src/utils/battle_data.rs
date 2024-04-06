use aes::cipher::{block_padding::NoPadding, BlockDecryptMut, KeyIvInit};
use anyhow::Result;
use serde_json::Value;

use super::crypto::md5::md5_digest;

const DEFAULT_LOGIN_TIME: u32 = 1672502400;
const LOG_TOKEN_KEY: &str = "pM6Umv*^hVQuB6t&";

type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

pub struct BattleDataDecoder {
    login_time: u32,
}

impl Default for BattleDataDecoder {
    fn default() -> Self {
        Self {
            login_time: DEFAULT_LOGIN_TIME,
        }
    }
}

impl BattleDataDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn login_time(mut self, login_time: u32) -> Self {
        self.login_time = login_time;
        self
    }

    pub fn decrypt_battle_data(&self, mut data: String) -> Result<Value> {
        let data = hex::decode(data.drain(..data.len() - 32))?;
        let mut src = LOG_TOKEN_KEY.to_string();
        src.push_str(&self.login_time.to_string());
        let key = md5_digest(src.as_bytes()).to_vec();
        let iv = hex::decode(&data)?;
        let aes = Aes128CbcDec::new(key.as_slice().into(), iv.as_slice().into());
        let res = aes.decrypt_padded_vec_mut::<NoPadding>(data.as_slice())?;
        let json_string = String::from_utf8(res)?;
        Ok(serde_json::from_str(json_string.trim())?)
    }
}
