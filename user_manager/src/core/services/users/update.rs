use crate::core::entity::user::{UpdateUserRequest, User};
use crate::core::services::users::UserService;
use std::error::Error;

impl UserService {
    pub fn update_user(&self, id: i64, req: UpdateUserRequest) -> Result<User, Box<dyn Error>> {
        // validate request
        let mut name: Option<&str> = None;
        if let Some(ref req_name) = req.name {
            if req_name.trim().is_empty() {
                return Err("Name cannot be empty".into());
            }
            name = Some(req_name.trim());
        }

        let mut email: Option<&str> = None;
        if let Some(ref req_email) = req.email {
            if !req_email.contains('@') {
                return Err("Invalid email format".into());
            }
            email = Some(req_email.trim());
        }

        self.repository.update(id, name, email)
    }
}
