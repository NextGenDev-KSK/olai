//! API-key storage in the Windows Credential Manager via `keyring`.
//! The key is never written to a file, an env dump, or a log.

use crate::error::AppResult;
use keyring::{Entry, Error as KeyringError};

/// Credential Manager service name.
const SERVICE: &str = "app.olai.desktop";
/// Credential Manager account/user name for the Anthropic key.
const ACCOUNT: &str = "anthropic-api-key";

fn entry() -> AppResult<Entry> {
    Ok(Entry::new(SERVICE, ACCOUNT)?)
}

/// Store the API key.
pub fn set_key(key: &str) -> AppResult<()> {
    entry()?.set_password(key)?;
    Ok(())
}

/// Retrieve the API key, or `None` if not set.
pub fn get_key() -> AppResult<Option<String>> {
    match entry()?.get_password() {
        Ok(key) => Ok(Some(key)),
        Err(KeyringError::NoEntry) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// Whether an API key is stored.
pub fn has_key() -> AppResult<bool> {
    Ok(get_key()?.is_some())
}

/// Delete the stored API key (idempotent).
pub fn delete_key() -> AppResult<()> {
    match entry()?.delete_credential() {
        Ok(()) | Err(KeyringError::NoEntry) => Ok(()),
        Err(e) => Err(e.into()),
    }
}
