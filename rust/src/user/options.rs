use std::path::PathBuf;

pub enum AppearanceOptions {
    Light,
    Dark,
    System,
}

pub struct AppSettings {
    pub appearance: AppearanceOptions,
    pub vault_path: Option<ValidVaultPath>,
}

struct ValidVaultPath(PathBuf);
