use std::fs::File;

use anyhow::Context as _;
use argh::FromArgs;
use sea_orm::{Database, DatabaseConnection, EntityTrait};

use crate::{
    data::{DatabaseData, User},
    db::prelude::*,
};

/// Import a database backup
#[derive(FromArgs)]
#[argh(subcommand, name = "generate")]
pub struct GenerateJsonCommand {}

impl GenerateJsonCommand {
    pub async fn run(self) -> anyhow::Result<()> {
        const DB_URL: &str = "mysql://root:my-secret-pw@localhost/aqn2013";

        println!("Connecting to database");

        let db: DatabaseConnection = Database::connect(DB_URL).await?;

        println!("Pinging db");
        db.ping().await.context("ping db")?;

        let users = MybbUsers::find()
            .all(&db)
            .await
            .context("read all users from db")?;
        let users = users
            .into_iter()
            .map(|user| {
                let user: User = user.try_into()?;

                Ok((user.id, user))
            })
            .collect::<anyhow::Result<_>>()?;

        let data = DatabaseData { users };

        let writer = File::create("data/output.json").context("open output file")?;
        serde_json::to_writer_pretty(writer, &data).context("write json output")?;

        println!("Done");

        Ok(())
    }
}
