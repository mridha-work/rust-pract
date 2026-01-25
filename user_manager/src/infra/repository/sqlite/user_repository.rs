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
                "INSERT INTO users (name, email) VALUES (?, ?)",
                &[name, email],
            )?;

            conn.last_insert_rowid()
        };

        self.find_by_id(id)?
            .ok_or_else(|| "User not found after creation".into())
    }

    fn find_by_id(&self, id: i64) -> Result<Option<User>, Box<dyn Error>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt =
            conn.prepare("SELECT id, name, email, created_at, updated_at FROM users WHERE id = ?")?;

        let user = stmt
            .query_row([id], |row| {
                Ok(User {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })
            .optional()?;

        Ok(user)
    }

    fn count_all(&self, keyword: Option<&str>) -> Result<i64, Box<dyn Error>> {
        let conn = self.conn.lock().unwrap();

        let mut query = String::from("SELECT COUNT(*) FROM users");
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![];

        if let Some(keyword) = keyword {
            query += " WHERE (LOWER(name) LIKE ? OR LOWER(email) LIKE ?)";
            let keyword_pattern = format!("%{}%", keyword.to_lowercase());
            params.push(Box::new(keyword_pattern.clone()));
            params.push(Box::new(keyword_pattern));
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
        keyword: Option<&str>,
    ) -> Result<Vec<User>, Box<dyn Error>> {
        let conn = self.conn.lock().unwrap();

        let mut query = String::from("SELECT id, name, email, created_at, updated_at FROM users");
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![];

        if let Some(keyword) = keyword {
            query += " WHERE (LOWER(name) LIKE ? OR LOWER(email) LIKE ?)";
            let meyword_pattern = format!("%{}%", keyword.to_lowercase());
            params.push(Box::new(meyword_pattern.clone()));
            params.push(Box::new(meyword_pattern));
        }

        query += " LIMIT ? OFFSET ?";
        params.push(Box::new(limit));
        params.push(Box::new(offset));

        let mut stmt = conn.prepare(&query)?;

        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let users = stmt
            .query_map(params_refs.as_slice(), |row| {
                Ok(User {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })?
            .collect::<Result<Vec<User>, rusqlite::Error>>()?;

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

            let mut update_clauses = vec![];
            let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![];

            if let Some(name) = name {
                update_clauses.push("name = ?");
                params.push(Box::new(name));
            }
            if let Some(email) = email {
                update_clauses.push("email = ?");
                params.push(Box::new(email));
            }

            if !update_clauses.is_empty() {
                update_clauses.push("updated_at = CURRENT_TIMESTAMP");

                let query = format!(
                    "UPDATE users SET {} WHERE id = ?",
                    update_clauses.join(", ")
                );

                params.push(Box::new(id));

                let params_refs: Vec<&dyn rusqlite::ToSql> =
                    params.iter().map(|p| p.as_ref()).collect();

                conn.execute(&query, params_refs.as_slice())?;
            }
        }

        self.find_by_id(id)?.ok_or_else(|| "User not found".into())
    }

    fn delete(&self, id: i64) -> Result<(), Box<dyn Error>> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM users WHERE id = ?", [id])?;
        Ok(())
    }
}
