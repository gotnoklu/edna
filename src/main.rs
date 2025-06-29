mod cli;
mod file_system;
mod templates;

use cli::{
    config::{CliMetadata, CliParserOptions},
    parse_cli_args, register_cli_args,
};

use templates::config::TemplatesMetadata;

fn main() {
    let templates_meta = TemplatesMetadata {
        directory: String::from("./templates"),
        filename: String::from("edna.config.json"),
    };

    let arg_matches = register_cli_args();

    parse_cli_args(CliParserOptions {
        metadata: &CliMetadata {
            templates_meta: &templates_meta,
        },
        matches: &arg_matches,
    });
}
