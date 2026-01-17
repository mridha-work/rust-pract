use crate::core::entity::user::{CreateUserRequest, User};
use crate::core::services::users::UserService;
use std::error::Error;

impl UserService {
    pub fn create_user(&self, req: CreateUserRequest) -> Result<User, Box<dyn Error>> {
        // validate request
        if req.name.trim().is_empty() {
            return Err("Name cannot be empty".into());
        }
        if !req.email.contains('@') {
            return Err("Invalid email format".into());
        }

        self.repository.create(&req.name.trim(), &req.email.trim())
    }
}
