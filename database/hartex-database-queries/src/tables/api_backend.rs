// ==================! DO NOT MODIFY !==================
// This file is automatically generated by `hartex-database-typedsql`. Please do not modify this in
// any way.
// ==================! DO NOT MODIFY !==================

use wtx::database::Record as _;
use wtx::database::client::postgres::Record;
pub struct StartTimestamps {
    component: String,
    timestamp: chrono::DateTime<chrono::offset::Utc>,
}
impl StartTimestamps {
    #[must_use]
    pub fn component(&self) -> &str {
        self.component.as_str()
    }
    #[must_use]
    pub fn timestamp(&self) -> chrono::DateTime<chrono::offset::Utc> {
        self.timestamp
    }
}
impl<'exec, E: From<wtx::Error>> TryFrom<Record<'exec, E>> for StartTimestamps
where
    crate::result::Error: From<E>,
{
    type Error = crate::result::Error;
    fn try_from(record: Record<'exec, E>) -> crate::result::Result<Self> {
        Ok(Self {
            component: record.decode("component")?,
            timestamp: record.decode("timestamp")?,
        })
    }
}
