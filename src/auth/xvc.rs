use sha2::{Digest, Sha512};
use std::ops::Deref;

use crate::config;

pub struct XVCKey {
    key: String,
}

impl XVCKey {
    pub fn new(email: &str, device_uuid: &str) -> XVCKey {
        XVCKey {
            key: hex::encode(Sha512::digest(format!("HEATH|{}|DEMIAN|{}|{}", config::AUTH_AGENT, email, device_uuid).as_bytes())),
        }
    }
}

impl Deref for XVCKey {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        return &self.key;
    }
}
