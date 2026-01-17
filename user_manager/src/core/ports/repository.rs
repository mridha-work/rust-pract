use crate::core::entity::user::User;
use std::error::Error;

pub trait UserRepository: Send + Sync {
    fn create(&self, name: &str, email: &str) -> Result<User, Box<dyn Error>>;
    fn find_by_id(&self, id: i64) -> Result<Option<User>, Box<dyn Error>>;
    fn count_all(&self, email_keyword: Option<&str>) -> Result<i64, Box<dyn Error>>;
    fn find_all(
        &self,
        limit: i64,
        offset: i64,
        email_keyword: Option<&str>,
    ) -> Result<Vec<User>, Box<dyn Error>>;
    fn update(
        &self,
        id: i64,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<User, Box<dyn Error>>;
    fn delete(&self, id: i64) -> Result<(), Box<dyn Error>>;
}
