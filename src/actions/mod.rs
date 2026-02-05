use argh::FromArgs;

mod generate_json;
mod render_html;

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum CliAction {
    GenerateJson(generate_json::GenerateJsonCommand),
    RenderHtml(render_html::RenderHtmlCommand),
}
