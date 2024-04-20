use async_trait::async_trait;
use uuid::Uuid;
use crate::handlers::config::{DBClient};
use crate::models::user_model::UserModel;
use crate::models::user_model::UserRole;

#[async_trait]
pub trait UserExt {
    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<UserModel>, sqlx::Error>;
    async fn get_users(&self, page: u32, limit: usize) -> Result<Vec<UserModel>, sqlx::Error>;
    async fn save_customer<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
    ) -> Result<UserModel, sqlx::Error>;

    async fn save_creator<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
    ) -> Result<UserModel, sqlx::Error>;
    async fn save_admin_user<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
    ) -> Result<UserModel, sqlx::Error>;

    async fn set_verification_code(
        &self,
        email: Option<&str>,
        verification_code: Option<String>,
        set_verification: String,
    ) -> Result<Option<UserModel>, sqlx::Error>;
}

#[async_trait]
impl UserExt for DBClient {
    async fn set_verification_code(
        &self,
        email: Option<&str>,
        verification_code: Option<String>,
        set_verification: String,
    ) -> Result<Option<UserModel>, sqlx::Error> {
        let user = if let Some(verification_code) = verification_code {
            let user = self.get_user(None, None, email).await?;
            if let Some(user) = user {
                if user.verification_code == verification_code.clone() {
                    sqlx::query_as!(
                      UserModel,
                    r#"
                    UPDATE users
                    SET verified = $1
                    WHERE email = $2
                    RETURNING id, name, email, password, photo, verified, verification_code, created_at, updated_at, role as "role: UserRole"
                    "#,
                    true,
                    email)
                        .fetch_optional(&self.pool)
                        .await?
                } else { None }
            } else { None }
        } else {
            sqlx::query_as!(
            UserModel,
            r#"UPDATE users SET verification_code = $1 WHERE email = $2 RETURNING id,name, email, password, photo,verified,verification_code,created_at,updated_at,role as "role: UserRole""#,
            &set_verification,
            email
        )
                .fetch_optional(&self.pool)
                .await?
        };
        Ok(user)
    }
    async fn get_user(
        &self,
        user_id: Option<uuid::Uuid>,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<UserModel>, sqlx::Error> {
        let mut user: Option<UserModel> = None;

        if let Some(user_id) = user_id {
            user = sqlx::query_as!(UserModel, r#"SELECT id,name, email, password, photo,verified,verification_code,created_at,updated_at,role as "role: UserRole" FROM users WHERE id = $1"#, user_id)
                .fetch_optional(&self.pool)
                .await?;
        } else if let Some(name) = name {
            user = sqlx::query_as!(UserModel, r#"SELECT id,name, email, password, photo,verified,verification_code,created_at,updated_at,role as "role: UserRole" FROM users WHERE name = $1"#, name)
                .fetch_optional(&self.pool)
                .await?;
        } else if let Some(email) = email {
            user = sqlx::query_as!(UserModel, r#"SELECT id,name, email, password, photo,verified,verification_code,created_at,updated_at,role as "role: UserRole" FROM users WHERE email = $1"#, email)
                .fetch_optional(&self.pool)
                .await?;
        }

        Ok(user)
    }
    async fn get_users(&self, page: u32, limit: usize) -> Result<Vec<UserModel>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        let users = sqlx::query_as!(
            UserModel,
            r#"SELECT id,name, email, password, photo,verified,verification_code,created_at,updated_at,role as "role: UserRole" FROM users
            LIMIT $1 OFFSET $2"#,
            limit as i64,
            offset as i64
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(users)
    }

    async fn save_customer<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
    ) -> Result<UserModel, sqlx::Error> {
        let user = sqlx::query_as!(
            UserModel,
            r#"INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id,name, email, password, photo,verified,verification_code,created_at,updated_at,role as "role: UserRole""#,
            name.into(),
            email.into(),
            password.into()
        )
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }
    async fn save_creator<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
    ) -> Result<UserModel, sqlx::Error> {
        let user = sqlx::query_as!(
            UserModel,
            r#"INSERT INTO users (name, email, password, role) VALUES ($1, $2, $3, $4) RETURNING id,name, email, password, photo,verified,verification_code,created_at,updated_at,role as "role: UserRole""#,
            name.into(),
            email.into(),
            password.into(),
            UserRole::Creator as UserRole
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(user)
    }
    async fn save_admin_user<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
    ) -> Result<UserModel, sqlx::Error> {
        let user = sqlx::query_as!(
               UserModel,
            r#"INSERT INTO users (name, email, password, role) VALUES ($1, $2, $3, $4) RETURNING id,name, email, password, photo,verified,verification_code,created_at,updated_at,role as "role: UserRole""#,
            name.into(),
            email.into(),
            password.into(),
            UserRole::Admin as UserRole
        )
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }
}
