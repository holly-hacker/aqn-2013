use std::{fs::File, path::Path};

use anyhow::Context as _;
use argh::FromArgs;
use askama::Template;

use crate::{
    data::DatabaseData,
    templates::{BaseProps, forum::ForumTemplate, index::IndexTemplate, thread::ThreadTemplate},
};

/// Import a database backup
#[derive(FromArgs)]
#[argh(subcommand, name = "render")]
pub struct RenderHtmlCommand {
    /// a base path to put in a `<base>` tag
    #[argh(option)]
    base_url: Option<String>,
}

impl RenderHtmlCommand {
    pub async fn run(self) -> anyhow::Result<()> {
        println!("Reading JSON data");
        let reader = File::open("data/output.json").context("open output file")?;
        let data: DatabaseData = serde_json::from_reader(reader).context("read database data")?;

        create_dir("./output").context("Create output dir")?;
        create_dir("./output/forums").context("Create forums dir")?;
        create_dir("./output/threads").context("Create threads dir")?;

        let base_props = BaseProps {
            base_url: self.base_url.unwrap_or_else(|| "/".into()),
        };

        println!("Copying static data");
        copy_dir_all("static", "output").context("copy static files")?;

        println!("Rendering index");
        let index_template = IndexTemplate::from((&data, &base_props));

        let mut output = String::new();
        index_template
            .render_into(&mut output)
            .context("render index")?;
        std::fs::write("output/index.html", output).context("write index")?;

        println!("Rendering {} forums", data.forums.len());
        for &forum_id in data.forums.keys() {
            let forum_template = ForumTemplate::try_from((&data, &base_props, forum_id))
                .context("create forum template")?;

            let mut output = String::new();
            forum_template
                .render_into(&mut output)
                .context("render forum page")?;
            std::fs::write(format!("output/forums/{forum_id}.html"), output)
                .context("write forum")?;
        }

        println!("Rendering {} threads", data.threads.len());
        for &thread_id in data.threads.keys() {
            let thread_template = ThreadTemplate::try_from((&data, &base_props, thread_id))
                .context("create thread template")?;

            let mut output = String::new();
            thread_template
                .render_into(&mut output)
                .context("render thread page")?;
            std::fs::write(format!("output/threads/{thread_id}.html"), output)
                .context("write thread")?;
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

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
