use crate::core::services::users::UserService;
use std::error::Error;

impl UserService {
    pub fn delete_user(&self, id: i64) -> Result<(), Box<dyn Error>> {
        self.repository.delete(id)
    }
}
