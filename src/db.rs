use crate::{
    models::{User, UserRole},
    utils::password,
};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuis;

#[derive(Debug, Clone)]
pub struct DBClient {
    pool: Pool<Postgres>,
}

impl DBClient {
    pub fn new(pool: Pool<Postgres>) -> Self {
        DBClient { pool }
    }
}

#[async_trait]
pub trait UserExt {
    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        name: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error>;

    async fn get_users(&self, page: u32, limit: usize) -> Result<Vec<User>, sqlx::Error>;

    async fn save_user<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
    ) -> Result<User, sqlx::Error>;

    async fn save_admin_user<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
    ) -> Result<User, sqlx::Error>;
}

#[async_trait]
impl UserExt for DBClient{
    async fn get_user(
            &self,
            user_id: Option<Uuid>,
            name: Option<&str>,
        ) -> impl Future<Output = Result<Option<User>, sqlx::Error>> {
        let mut user: Option<User> = None;
        if let Some(user_id) = user_id{
            user=sqlx::query_as!(
                User, r#"SELECT id, name, email, password, photo, verified, created_at, updated_at, role as "role: UserRole" FROM users where id=$1"#, user_id
            ).fetch_optional(&self.pool).await?;
        }else if let Some(name) = name{
            user=sqlx::query_as!(
                User, r#"SELECT id, name, email, password, photo, verified, created_at, updated_at, role as "role: UserRole" FROM users where id=$1"#, name
            ).fetch_optional(&self.pool).await?;
        }else if let Some(email) = email{
            user=sqlx::query_as!(
                User, r#"SELECT id, name, email, password, photo, verified, created_at, updated_at, role as "role: UserRole" FROM users where id=$1"#, email
            ).fetch_optional(&self.pool).await?;
        }
        Ok(user)
    }
} 
