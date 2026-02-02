use std::path::{Path, PathBuf};
use leptos_i18n_build::{Config, TranslationsInfos};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::new("en")?;

    let translations_infos = TranslationsInfos::parse(cfg)?;

    translations_infos.emit_diagnostics();
    translations_infos.rerun_if_locales_changed();
    translations_infos.generate_i18n_module(PathBuf::from("./locales"))?;

    Ok(())
}