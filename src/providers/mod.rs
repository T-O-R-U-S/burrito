/*
 * Copyright (c) 2024.
 *
 * Licensed under the MIT license <http://opensource.org/licenses/MIT>.
 */
use crate::database::Entry;

pub mod sensitive_text;
pub mod burrito_box;
pub mod recursive;
pub mod burrito_box_sym;

pub trait Provider: Sized {
    fn name() -> String;
    fn version() -> String;
    fn into_entry(self) -> Entry;
    fn from_entry(entry: Entry) -> anyhow::Result<Self>;

    fn verify_version(cmp: &Entry) -> anyhow::Result<()> {
        use anyhow::bail;

        let cmp = cmp.get_str("version")?;

        if cmp != Self::version() {
            bail!("Version mismatch: expected {}, got {}", Self::version(), cmp);
        }

        Ok(())
    }
}