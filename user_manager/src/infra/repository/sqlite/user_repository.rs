use crate::core::entity::user::User;
use crate::core::ports::repository::UserRepositoryPort;
use rusqlite::{Connection, OptionalExtension};
use std::error::Error;
use std::sync::{Arc, Mutex};

pub struct SqliteUserRepository {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteUserRepository {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}

impl UserRepositoryPort for SqliteUserRepository {
    fn create(&self, name: &str, email: &str) -> Result<User, Box<dyn Error>> {
        let id = {
            let conn = self.conn.lock().unwrap();
            conn.execute(
                "INSERT INTO users (name, email) VALUES (?1, ?2)",
                &[name, email],
            )?;

            conn.last_insert_rowid()
        };

        self.find_by_id(id)?
            .ok_or_else(|| "User not found after creation".into())
    }

    fn find_by_id(&self, id: i64) -> Result<Option<User>, Box<dyn Error>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, name, email, created_at, updated_at FROM users WHERE id = ?1")?;

        let user = stmt
            .query_row([id], |row| {
                Ok(User {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    email: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })
            .optional()?;

        Ok(user)
    }

    fn count_all(&self, email_keyword: Option<&str>) -> Result<i64, Box<dyn Error>> {
        let conn = self.conn.lock().unwrap();

        let mut query = String::from("SELECT COUNT(*) FROM users");
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![];

        if let Some(email) = email_keyword {
            query += " WHERE LOWER(email) LIKE ?";
            let email_pattern = format!("%{}%", email.to_lowercase());
            params.push(Box::new(email_pattern));
        }

        let mut stmt = conn.prepare(&query)?;

        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let user_count = stmt.query_row(params_refs.as_slice(), |row| row.get(0))?;

        Ok(user_count)
    }

    fn find_all(
        &self,
        limit: i64,
        offset: i64,
        email_keyword: Option<&str>,
    ) -> Result<Vec<User>, Box<dyn Error>> {
        let conn = self.conn.lock().unwrap();

        let mut query = String::from("SELECT id, name, email, created_at, updated_at FROM users");
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![];

        if let Some(email) = email_keyword {
            query += " WHERE LOWER(email) LIKE ?";
            let email_pattern = format!("%{}%", email.to_lowercase());
            params.push(Box::new(email_pattern));
        }

        query += " LIMIT ? OFFSET ?";
        params.push(Box::new(limit));
        params.push(Box::new(offset));

        let mut stmt = conn.prepare(&query)?;

        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let users = stmt
            .query_map(params_refs.as_slice(), |row| {
                Ok(User {
                    id: Some(row.get(0)?),
                    name: row.get(1)?,
                    email: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(users)
    }

    fn update(
        &self,
        id: i64,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<User, Box<dyn Error>> {
        {
            let conn = self.conn.lock().unwrap();

            if let Some(name) = name {
                conn.execute(
                    "UPDATE users SET name = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
                    &[name, &id.to_string()],
                )?;
            }
            if let Some(email) = email {
                conn.execute(
                    "UPDATE users SET email = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2",
                    &[email, &id.to_string()],
                )?;
            }
        }

        self.find_by_id(id)?.ok_or_else(|| "User not found".into())
    }

    fn delete(&self, id: i64) -> Result<(), Box<dyn Error>> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM users WHERE id = ?1", [id])?;
        Ok(())
    }
}
