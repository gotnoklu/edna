use clap::{Arg, ArgAction, ArgMatches, Command, builder::BoolValueParser, command};
use config::CliParserOptions;
use project_cli::run_new_project_cli_args;
use template_cli::run_new_template_cli_args;

pub mod config;
pub mod project_cli;
pub mod template_cli;
pub mod theme;

pub fn register_cli_args() -> ArgMatches {
    let arg_matches =
        command!()
            .subcommand(
                Command::new("new")
                    .subcommand(
                        Command::new("project")
                            .about("Creates a new project")
                            .arg(
                                Arg::new("name")
                                    .short('n')
                                    .long("name")
                                    .help("The name of the project to be created"),
                            )
                            .arg(
                                Arg::new("description")
                                    .short('d')
                                    .long("desc")
                                    .help("The description of the project."),
                            )
                            .arg(
                                Arg::new("version")
                                    .short('v')
                                    .long("version")
                                    .help("The version of the project."),
                            )
                            .arg(
                                Arg::new("author")
                                    .short('a')
                                    .long("author")
                                    .help("The author of the project."),
                            )
                            .arg(
                                Arg::new("output")
                                    .short('o')
                                    .long("output")
                                    .help("Sets a custom path where the project will be created"),
                            )
                            .arg(
                                Arg::new("template")
                                    .short('t')
                                    .long("template")
                                    .help("Sets the path to the template for the new project."),
                            )
                            .arg(
                                Arg::new("create_empty")
                                    .short('e')
                                    .long("empty")
                                    .value_parser(BoolValueParser::new())
                                    .help("Creates an empty project."),
                            ),
                    )
                    .subcommand(
                        Command::new("template")
                            .arg(Arg::new("source").short('s').long("src").help(
                                "The source path for the template. It can be a local path or a URL",
                            ))
                            .arg(Arg::new("output").short('o').long("output").help(
                                "Where in the templates directory to create the new template.",
                            ))
                            .arg(
                                Arg::new("name")
                                    .short('n')
                                    .long("name")
                                    .help("The name of the template."),
                            )
                            .arg(
                                Arg::new("description")
                                    .short('d')
                                    .long("desc")
                                    .help("The description of the template."),
                            )
                            .arg(
                                Arg::new("version")
                                    .short('v')
                                    .long("version")
                                    .help("The version of the template."),
                            )
                            .arg(
                                Arg::new("author")
                                    .short('a')
                                    .long("author")
                                    .help("The author of the template."),
                            )
                            .arg(
                                Arg::new("exclude_config")
                                    .short('e')
                                    .long("exclude-config")
                                    .help("Exclude the config file when copying the template.")
                                    .num_args(0)
                                    .value_parser(BoolValueParser::new()),
                            )
                            .arg(
                                Arg::new("exclude_paths")
                                    .short('p')
                                    .long("exclude-paths")
                                    .help("Exclude certain paths when copying the template.")
                                    .action(ArgAction::Append),
                            )
                            .arg(
                                Arg::new("scripts")
                                    .short('i')
                                    .long("script")
                                    .help("Add initialisation scripts to run when the project is created.")
                                    .action(ArgAction::Append),
                            ),
                    ),
            )
            .get_matches();

    arg_matches
}

pub fn parse_cli_args(options: CliParserOptions) {
    if let Some(new_cmd) = options.matches.subcommand_matches("new") {
        if let Some(_) = new_cmd.subcommand_matches("project") {
            return run_new_project_cli_args(&CliParserOptions {
                metadata: &options.metadata,
                matches: &new_cmd,
            });
        } else if let Some(_) = new_cmd.subcommand_matches("template") {
            return run_new_template_cli_args(&CliParserOptions {
                metadata: &options.metadata,
                matches: &new_cmd,
            });
        }
    }
}
