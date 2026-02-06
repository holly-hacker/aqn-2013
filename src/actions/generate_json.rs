use std::{collections::BTreeMap, fs::File};

use anyhow::Context as _;
use argh::FromArgs;
use sea_orm::{
    Database, DatabaseConnection, EntityTrait, ModelTrait, PrimaryKeyTrait, Value,
    sea_query::ValueTuple,
};

use crate::{data::DatabaseData, db::prelude::*};

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

        println!("Reading database");
        let users = Self::get_from_db::<MybbUsers, _>(&db)
            .await
            .context("get users")?;

        let forums = Self::get_from_db::<MybbForums, _>(&db)
            .await
            .context("get forums")?;

        let threads = Self::get_from_db::<MybbThreads, _>(&db)
            .await
            .context("get threads")?;

        let posts = Self::get_from_db::<MybbPosts, _>(&db)
            .await
            .context("get forums")?;

        let data = DatabaseData {
            users,
            forums,
            threads,
            posts,
        };

        println!("Writing to disk");
        let writer = File::create("data/output.json").context("open output file")?;
        serde_json::to_writer_pretty(writer, &data).context("write json output")?;

        println!("Done");

        Ok(())
    }

    async fn get_from_db<TDb: EntityTrait, TModel: TryFrom<TDb::Model, Error = anyhow::Error>>(
        db: &DatabaseConnection,
    ) -> anyhow::Result<BTreeMap<<TDb::PrimaryKey as PrimaryKeyTrait>::ValueType, TModel>>
    where
        <TDb::PrimaryKey as PrimaryKeyTrait>::ValueType: Ord + PkConvert,
    {
        let db_entities = TDb::find()
            .all(db)
            .await
            .context("read database entities")?;
        let models = db_entities
            .into_iter()
            .map(|f| {
                let pk_value = match f.get_primary_key_value() {
                    ValueTuple::One(one) => one,
                    _ => anyhow::bail!("Expected single primary key"),
                };
                let pk =
                    <<TDb::PrimaryKey as PrimaryKeyTrait>::ValueType as PkConvert>::try_convert(
                        pk_value,
                    );
                let m: TModel = f.try_into().context("ya")?;
                Ok((pk, m))
            })
            .collect::<anyhow::Result<_>>()
            .context("convert models")?;

        Ok(models)
    }
}

pub trait PkConvert {
    fn try_convert(value: Value) -> Self;
}

impl PkConvert for u16 {
    fn try_convert(value: Value) -> Self {
        match value {
            Value::SmallUnsigned(Some(x)) => x,
            Value::SmallUnsigned(None) => unreachable!("Null primary key"),
            _ => unreachable!("Unexepected primary key type"),
        }
    }
}

impl PkConvert for u32 {
    fn try_convert(value: Value) -> Self {
        match value {
            Value::Unsigned(Some(x)) => x,
            Value::Unsigned(None) => unreachable!("Null primary key"),
            _ => unreachable!("Unexepected primary key type"),
        }
    }
}
