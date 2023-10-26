//! Easy access to files

use super::for_crate::std::ExtendedStdResult;
use serde::{de::DeserializeOwned, Serialize};

/// Write value to file with pretty RON. All errors are returned as strings.
pub fn save_ron_file<T: Serialize>(value: &T, filename: &str) -> bool {
    ron::ser::to_string_pretty(value, Default::default())
        .map_err_to_string()
        .and_then(|data| std::fs::write(&filename, data).map_err_to_string())
        .map_err(|e| format!("{} [file \"{}\"]", e.to_string(), filename))
        .ok_or_log_err()
        .is_some()
}

/// Read value from RON file. All errors are returned as strings.
pub fn load_ron_file<T: DeserializeOwned>(filename: &str) -> Option<T> {
    std::fs::read_to_string(&filename)
        .map_err_to_string()
        .and_then(|data| ron::from_str(&data).map_err_to_string())
        .map_err(|e| format!("{} [file \"{}\"]", e.to_string(), filename))
        .ok_or_log_err()
}

/// Read value from RON file, on error consider it to be default-initialized.
///
/// All errors are logged.
///
/// File is written back after reading; this is useful to automatically create
/// config files after first launch and (together with `serde(default)`) to
/// update them if struct definition changes.
pub fn load_and_update_ron_file<T: Serialize + DeserializeOwned + Default>(filename: &str) -> T {
    let value = load_ron_file(filename).unwrap_or_default();
    save_ron_file(&value, filename);
    value
}
