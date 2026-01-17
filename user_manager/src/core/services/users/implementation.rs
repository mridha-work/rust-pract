use crate::core::entity::user::{
    CreateUserRequest, ListUsersQueryParam, Pagination, UpdateUserRequest, User, UserList,
};
use crate::core::ports::service::UserServicePort;
use crate::core::services::users::UserService;
use std::error::Error;

impl UserServicePort for UserService {
    fn create_user(&self, req: CreateUserRequest) -> Result<User, Box<dyn Error>> {
        // validate request
        if req.name.trim().is_empty() {
            return Err("Name cannot be empty".into());
        }
        if !req.email.contains('@') {
            return Err("Invalid email format".into());
        }

        self.repository.create(&req.name.trim(), &req.email.trim())
    }

    fn get_user_by_id(&self, id: i64) -> Result<Option<User>, Box<dyn Error>> {
        self.repository.find_by_id(id)
    }

    fn list_users(&self, query: ListUsersQueryParam) -> Result<UserList, Box<dyn Error>> {
        // validate query param
        let mut limit = 100;
        if let Some(q_limit) = query.limit {
            if q_limit > 0 && q_limit <= 1000 {
                limit = q_limit
            }
        }

        let mut offset = 0;
        if let Some(q_offset) = query.offset {
            if q_offset >= 0 {
                offset = q_offset
            }
        }

        let mut email_keyword: Option<&str> = None;
        if let Some(ref q_email) = query.email {
            if !q_email.trim().is_empty() {
                email_keyword = Some(q_email.trim());
            }
        }

        // get total users
        let total_users = self.repository.count_all(email_keyword)?;

        // early return if zero
        if total_users == 0 {
            return Ok(UserList {
                items: vec![],
                pagination: Pagination {
                    page: 1,
                    total_pages: 0,
                    total_items: 0,
                },
            });
        }

        // get users
        let users = self.repository.find_all(limit, offset, email_keyword)?;

        // calculate pagination
        let current_page = (offset / limit) + 1;
        let total_pages = (total_users as f64 / limit as f64).ceil() as i64;

        Ok(UserList {
            items: users,
            pagination: Pagination {
                page: current_page,
                total_pages: total_pages,
                total_items: total_users,
            },
        })
    }

    fn update_user(&self, id: i64, req: UpdateUserRequest) -> Result<User, Box<dyn Error>> {
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

    fn delete_user(&self, id: i64) -> Result<(), Box<dyn Error>> {
        self.repository.delete(id)
    }
}
