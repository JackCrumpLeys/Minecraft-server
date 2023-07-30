use crate::db::DB;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub(crate) username: String,
    pub(crate) uuid: String,
    pub(crate) visited_count: usize,
}

impl Player {
    pub(crate) async fn get_or_create(
        username: String,
        uuid: String,
        db_helper: &DB,
    ) -> surrealdb::Result<Self> {
        let db = db_helper.db.as_ref().unwrap();

        if let Some(player) = db
            .select(("player", uuid.clone()))
            .await?
        {
            return Ok(player);
        }

        let player = Player {
            username,
            uuid,
            visited_count: 0,
        };

        player.save(db_helper).await?;

        Ok(player)
    }

    pub(crate) async fn save(&self, db_helper: &DB) -> surrealdb::Result<()> {
        db_helper
            .db
            .as_ref()
            .unwrap()
            .update::<Option<Player>>(("player", self.uuid.clone()))
            .content::<Player>(self.clone())
            .await.unwrap();

        Ok(())
    }
}
