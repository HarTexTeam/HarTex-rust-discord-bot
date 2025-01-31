// ==================! DO NOT MODIFY !==================
// This file is automatically generated by `hartex-database-typedsql`. Please do not modify this in
// any way.
// ==================! DO NOT MODIFY !==================

use wtx::database::Record as _;
use wtx::database::client::postgres::Record;
pub struct NightlyGuildConfigurations {
    enabled_plugins: Vec<String>,
    guild_id: String,
}
impl NightlyGuildConfigurations {
    #[must_use]
    pub fn enabled_plugins(&self) -> &[String] {
        self.enabled_plugins.as_slice()
    }
    #[must_use]
    pub fn guild_id(&self) -> &str {
        self.guild_id.as_str()
    }
}
impl<'exec, E: From<wtx::Error>> TryFrom<Record<'exec, E>> for NightlyGuildConfigurations
where
    crate::result::Error: From<E>,
{
    type Error = crate::result::Error;
    fn try_from(record: Record<'exec, E>) -> crate::result::Result<Self> {
        Ok(Self {
            enabled_plugins: record.decode("enabled_plugins")?,
            guild_id: record.decode("guild_id")?,
        })
    }
}
