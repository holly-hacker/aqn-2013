use std::{fs::File, path::Path};

use anyhow::Context as _;
use argh::FromArgs;
use askama::Template;

use crate::{
    data::DatabaseData,
    templates::{forum::ForumTemplate, index::IndexTemplate},
};

/// Import a database backup
#[derive(FromArgs)]
#[argh(subcommand, name = "render")]
pub struct RenderHtmlCommand {}

impl RenderHtmlCommand {
    pub async fn run(self) -> anyhow::Result<()> {
        let reader = File::open("data/output.json").context("open output file")?;
        let data: DatabaseData = serde_json::from_reader(reader).context("read database data")?;

        create_dir("./output").context("Create output dir")?;
        create_dir("./output/forums").context("Create forums dir")?;

        let index_template = IndexTemplate::from(&data);

        let mut output = String::new();
        index_template
            .render_into(&mut output)
            .context("render index")?;
        std::fs::write("output/index.html", output).context("write index")?;

        for &forum_id in data.forums.keys() {
            let forum_template =
                ForumTemplate::try_from((&data, forum_id)).context("create forum template")?;

            let mut output = String::new();
            forum_template
                .render_into(&mut output)
                .context("render forum page")?;
            std::fs::write(format!("output/forums/{forum_id}.html"), output)
                .context("write forum")?;
        }

        Ok(())
    }
}

fn create_dir(p: impl AsRef<Path> + Copy) -> anyhow::Result<()> {
    if std::fs::exists(p).context("check dir exists")? {
        std::fs::remove_dir_all(p).context("remove dir")?;
    }
    std::fs::create_dir_all(p).context("create dir")?;

    Ok(())
}
