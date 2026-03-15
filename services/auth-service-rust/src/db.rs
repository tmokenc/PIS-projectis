use bcrypt::{hash, DEFAULT_COST};
use std::path::Path;

use crate::config::Config;
use surrealdb::{
    engine::local::{Db as LocalDb, SurrealKv},
    Surreal,
};

use crate::models::{NewUserRecord, RevokedTokenRecord, UserRecord};

#[derive(Clone)]
pub struct Db {
    inner: Surreal<LocalDb>,
}

impl Db {
    pub async fn connect(config: &Config) -> anyhow::Result<Self> {
        let db = Surreal::new::<SurrealKv>(config.db_path.clone()).await?;
        db.use_ns(&config.surreal_namespace)
            .use_db(&config.surreal_database)
            .await?;

        Ok(Self { inner: db })
    }

    pub async fn seed_demo_users(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        #[derive(serde::Deserialize)]
        struct DemoUser {
            email: String,
            password: String,
            role: String,
            firstname: String,
            lastname: String,
        }

        let toml_str = std::fs::read_to_string(path)?;
        let parsed: std::collections::HashMap<String, Vec<DemoUser>> = toml::from_str(&toml_str)?;
        let demo_users = parsed.into_values().flatten().collect::<Vec<DemoUser>>();

        for user in demo_users {
            if self.find_user_by_email(&user.email).await?.is_some() {
                continue;
            }

            let new_user = NewUserRecord {
                firstname: user.firstname,
                lastname: user.lastname,
                email: user.email,
                password_hash: hash(user.password, DEFAULT_COST)?,
                role: user.role,
            };

            self.insert_user(&new_user).await?;
            tracing::info!(new_user.email, new_user.role, "seeded demo user");
        }

        Ok(())
    }

    pub async fn find_user_by_email(&self, email: &str) -> anyhow::Result<Option<UserRecord>> {
        let mut result = self
            .inner
            .query("SELECT * FROM user WHERE email = $email LIMIT 1;")
            .bind(("email", email.to_string()))
            .await?;

        let users: Vec<UserRecord> = result.take(0)?;
        Ok(users.into_iter().next())
    }

    pub async fn find_user_by_id(&self, user_id: &str) -> anyhow::Result<Option<UserRecord>> {
        self.inner
            .select(("user", user_id))
            .await
            .map_err(|e| e.into())
    }

    pub async fn insert_user(&self, user: &NewUserRecord) -> anyhow::Result<UserRecord> {
        self.inner
            .create("user")
            .content(user.clone())
            .await?
            .ok_or_else(|| anyhow::anyhow!("failed to create user"))
    }

    pub async fn revoke_token(&self, token: &str) -> anyhow::Result<()> {
        let rec = RevokedTokenRecord {
            id: uuid::Uuid::new_v4().to_string(),
            token: token.to_owned(),
        };

        let _: Option<RevokedTokenRecord> = self
            .inner
            .create(("revoked_token", rec.id.clone()))
            .content(rec)
            .await?;

        Ok(())
    }

    pub async fn is_token_revoked(&self, token: &str) -> anyhow::Result<bool> {
        let mut result = self
            .inner
            .query("SELECT * FROM revoked_token WHERE token = $lookup_token LIMIT 1;")
            .bind(("lookup_token", token.to_owned()))
            .await?;

        let rows: Vec<RevokedTokenRecord> = result.take(0)?;
        Ok(!rows.is_empty())
    }
}
