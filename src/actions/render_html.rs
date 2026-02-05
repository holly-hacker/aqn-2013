use std::fs::File;

use anyhow::Context as _;
use argh::FromArgs;
use askama::Template;

use crate::{data::DatabaseData, templates::index::IndexTemplate};

/// Import a database backup
#[derive(FromArgs)]
#[argh(subcommand, name = "render")]
pub struct RenderHtmlCommand {}

impl RenderHtmlCommand {
    pub async fn run(self) -> anyhow::Result<()> {
        let reader = File::open("data/output.json").context("open output file")?;
        let data: DatabaseData = serde_json::from_reader(reader).context("read database data")?;

        if std::fs::exists("./output").context("check output exists")? {
            std::fs::remove_dir_all("./output").context("remove output dir")?;
        }
        std::fs::create_dir_all("./output").context("create output dir")?;

        let index_template = IndexTemplate::from(&data);

        let mut output = String::new();
        index_template
            .render_into(&mut output)
            .context("render index")?;
        std::fs::write("output/index.html", output).context("write index")?;

        Ok(())
    }
}
