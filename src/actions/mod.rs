use argh::FromArgs;

mod generate_json;

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum CliAction {
    GenerateJson(generate_json::GenerateJsonCommand),
}
