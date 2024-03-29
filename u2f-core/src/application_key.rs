use serde_derive::{Deserialize, Serialize};

use crate::app_id::AppId;
use crate::key_handle::KeyHandle;
use crate::private_key::PrivateKey;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ApplicationKey {
    pub application: AppId,
    pub handle: KeyHandle,
    key: PrivateKey,
}

impl ApplicationKey {
    pub fn new(application: AppId, handle: KeyHandle, key: PrivateKey) -> ApplicationKey {
        ApplicationKey {
            application,
            handle,
            key,
        }
    }
    pub(crate) fn key(&self) -> &PrivateKey {
        &self.key
    }
}
