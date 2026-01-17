use crate::core::entity::user::{
    CreateUserRequest, ListUsersQueryParam, UpdateUserRequest, User, UserList,
};
use std::error::Error;

pub trait UserServicePort: Send + Sync {
    fn create_user(&self, req: CreateUserRequest) -> Result<User, Box<dyn Error>>;
    fn get_user_by_id(&self, id: i64) -> Result<Option<User>, Box<dyn Error>>;
    fn list_users(&self, query: ListUsersQueryParam) -> Result<UserList, Box<dyn Error>>;
    fn update_user(&self, id: i64, req: UpdateUserRequest) -> Result<User, Box<dyn Error>>;
    fn delete_user(&self, id: i64) -> Result<(), Box<dyn Error>>;
}
