//! A simple notification store using the `fjall` embedded database.
//! This does not support full CRUD operations, as there is no need
//! to update or delete notifications in our use case.
//! Instead, we only support creating notifications, listing them for a user,
//! and marking them as read.
//!
//! That why it is called a "store" instead of a "database".

use chrono::Utc;
use fjall::KeyspaceCreateOptions;
use fjall::{Database, Keyspace, PersistMode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationRecord {
    pub id: String,
    pub user_id: String,
    pub message: String,
    pub date: chrono::DateTime<Utc>,
    pub read: bool,
}

#[derive(Clone)]
pub struct NotificationStore {
    db: Database,
    notifications: Keyspace,
    user_index: Keyspace,
}

impl NotificationStore {
    pub fn open(path: &str) -> anyhow::Result<Self> {
        let db = Database::builder(path).open()?;
        let notifications = db.keyspace("notifications", KeyspaceCreateOptions::default)?;
        let user_index = db.keyspace("user_notifications", KeyspaceCreateOptions::default)?;
        Ok(Self {
            db,
            notifications,
            user_index,
        })
    }

    fn user_index_key(user_id: &str, date: chrono::DateTime<Utc>, notification_id: &str) -> String {
        let millis = date.timestamp_millis().max(0) as u64;
        let reverse_millis = u64::MAX - millis;
        format!("{}:{:020}:{}", user_id, reverse_millis, notification_id)
    }

    pub fn save_notification(&self, record: &NotificationRecord) -> anyhow::Result<()> {
        let value = serde_json::to_vec(record)?;
        self.notifications.insert(record.id.as_bytes(), value)?;
        let index_key = Self::user_index_key(&record.user_id, record.date, &record.id);
        self.user_index
            .insert(index_key.as_bytes(), record.id.as_bytes())?;
        self.db.persist(PersistMode::SyncAll)?;
        Ok(())
    }

    pub fn get_notification(&self, id: &str) -> anyhow::Result<Option<NotificationRecord>> {
        let Some(bytes) = self.notifications.get(id.as_bytes())? else {
            return Ok(None);
        };
        Ok(Some(serde_json::from_slice(&bytes)?))
    }

    pub fn list_notifications(&self, user_id: &str) -> anyhow::Result<Vec<NotificationRecord>> {
        let mut rows = Vec::new();
        let prefix = format!("{}:", user_id);
        for entry in self.user_index.prefix(prefix.as_bytes()) {
            let value = entry.value()?;
            let notification_id = std::str::from_utf8(&value)?;
            if let Some(record) = self.get_notification(notification_id)? {
                rows.push(record);
            }
        }
        Ok(rows)
    }

    pub fn mark_as_read(&self, notification_id: &str) -> anyhow::Result<()> {
        let Some(mut record) = self.get_notification(notification_id)? else {
            return Ok(());
        };
        record.read = true;
        let value = serde_json::to_vec(&record)?;
        self.notifications.insert(record.id.as_bytes(), value)?;
        self.db.persist(PersistMode::SyncAll)?;
        Ok(())
    }
}
