use std::path;

use console::{Emoji, style};
use dialoguer::{Input, Select};
use whoami;

use crate::templates::config::CreatedTemplateConfig;

use super::{config::CliParserOptions, theme::CliTheme};

pub fn run_new_template_cli_args(options: &CliParserOptions) {
    let template_cmd = options.matches.subcommand_matches("template").unwrap();
    let templates_directory = &options.metadata.templates_meta.directory;

    let template_name = if let Some(name) = template_cmd.get_one::<String>("name") {
        name
    } else {
        let input: String = Input::with_theme(&CliTheme::default())
            .with_prompt("Enter the template's name (Required)")
            .with_post_completion_text("Template Name")
            .allow_empty(false)
            .interact()
            .unwrap();

        &input.to_string()
    };

    let template_author = if let Some(author) = template_cmd.get_one::<String>("author") {
        author
    } else {
        let input: String = Input::with_theme(&CliTheme::default())
            .with_prompt("Enter the template's author")
            .with_post_completion_text("Template Author")
            .allow_empty(true)
            .show_default(true)
            .default(String::from(whoami::realname()))
            .interact()
            .unwrap();

        &input.to_string()
    };

    let template_version = if let Some(version) = template_cmd.get_one::<String>("version") {
        version
    } else {
        let input: String = Input::with_theme(&CliTheme::default())
            .with_prompt("Enter the template's version")
            .with_post_completion_text("Template Version")
            .allow_empty(true)
            .show_default(true)
            .default(String::from("1.0.0"))
            .interact()
            .unwrap();

        &input.to_string()
    };

    let template_description =
        if let Some(description) = template_cmd.get_one::<String>("description") {
            description
        } else {
            let input: String = Input::with_theme(&CliTheme::default())
                .with_prompt("Enter the template's description")
                .with_post_completion_text("Template Description")
                .allow_empty(true)
                .interact()
                .unwrap();

            &input.to_string()
        };

    let template_exclude_config: bool =
        if let Some(exclude_config) = template_cmd.get_one::<bool>("exclude_config") {
            exclude_config.clone()
        } else {
            let input: usize = Select::with_theme(&CliTheme::default())
                .with_prompt("Ignore the template's config when creating project")
                .default(0)
                .items(&vec![String::from("Yes"), String::from("No")])
                .interact()
                .unwrap();

            input == 0
        };

    let template_excluded_paths: Vec<String> =
        if let Some(excluded_paths) = template_cmd.get_many::<String>("exclude_paths") {
            excluded_paths
                .map(|p| String::from(p))
                .collect::<Vec<String>>()
        } else {
            let input: String = Input::with_theme(&CliTheme::default())
                .with_prompt("Ignore certain paths when copying the template")
                .with_post_completion_text("Excluded Template Paths")
                .allow_empty(true)
                .interact()
                .unwrap();

            if input == "" {
                vec![]
            } else {
                input.split(",").map(|i| String::from(i)).collect()
            }
        };

    let template_scripts: Vec<String> =
        if let Some(scripts) = template_cmd.get_many::<String>("scripts") {
            scripts.map(|p| String::from(p)).collect::<Vec<String>>()
        } else {
            let input: String = Input::with_theme(&CliTheme::default())
                .with_prompt("Add initialisation scripts for the project separated by a comma")
                .with_post_completion_text("Initialisation Scripts")
                .allow_empty(true)
                .interact()
                .unwrap();

            if input == "" {
                vec![]
            } else {
                input.split(",").map(|i| String::from(i)).collect()
            }
        };

    let template_output = if let Some(output) = template_cmd.get_one::<String>("output") {
        &path::absolute(format!("{}/{}", templates_directory, output))
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    } else {
        let default_path = String::from(format!("{}/{}", templates_directory, template_name));

        let input: String = Input::with_theme(&CliTheme::default())
            .with_prompt("Where in the templates directory to create it")
            .with_post_completion_text("Template Output")
            .show_default(true)
            .default(default_path.clone())
            .interact()
            .unwrap();

        if input == default_path {
            &input.to_string()
        } else {
            &path::absolute(format!("{}/{}", templates_directory, input))
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        }
    };

    let template_source = if let Some(source) = template_cmd.get_one::<String>("source") {
        if source == "" {
            &source.to_string()
        } else {
            &path::absolute(source)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        }
    } else {
        let input: String = Input::with_theme(&CliTheme::default())
            .with_prompt("The folder to be used when creating the template")
            .with_post_completion_text("Template Source")
            .allow_empty(true)
            .interact()
            .unwrap();

        if input == "" {
            &input.to_string()
        } else {
            &path::absolute(input).unwrap().to_str().unwrap().to_string()
        }
    };

    let template_config = CreatedTemplateConfig {
        name: template_name.clone(),
        target: String::from("project"),
        author: template_author.clone(),
        version: template_version.clone(),
        description: template_description.clone(),
        exclude_config: template_exclude_config,
        exclude_paths: template_excluded_paths,
        scripts: template_scripts,
    };

    let _ = CreatedTemplateConfig::create_template(
        &template_output,
        &template_source,
        &template_config,
        &options.metadata.templates_meta,
    );

    println!(
        "\n{} {}",
        style(Emoji("🚀", ":-)")).green().bright(),
        style("Template created!").yellow().bold(),
    );
}
