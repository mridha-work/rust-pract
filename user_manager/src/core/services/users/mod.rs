mod create;
mod delete;
mod read;
mod update;
use crate::core::ports::repository::UserRepository;
use std::sync::Arc;
pub struct UserService {
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
}
