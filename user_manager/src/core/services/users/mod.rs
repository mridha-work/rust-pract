mod implementation;

use crate::core::ports::repository::UserRepositoryPort;
use std::sync::Arc;

pub struct UserService {
    repository: Arc<dyn UserRepositoryPort>,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepositoryPort>) -> Self {
        Self { repository }
    }
}
