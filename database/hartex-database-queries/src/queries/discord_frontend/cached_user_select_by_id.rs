// ==================! DO NOT MODIFY !==================
// This file is automatically generated by `hartex-database-typedsql`. Please do not modify this in
// any way.
// ==================! DO NOT MODIFY !==================

pub struct CachedUserSelectById {
    id: String,
}
impl CachedUserSelectById {
    #[must_use = "Queries must be executed after construction"]
    pub fn bind(id: String) -> Self {
        Self { id }
    }
}
