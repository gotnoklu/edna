use std::{
    fs,
    io::{self, Write},
    path::{self, PathBuf},
    process::{self, Command},
    time::Duration,
};

use console::{Emoji, style};
use dialoguer::{Input, Select};
use indicatif::ProgressBar;

use super::config::CliParserOptions;
use super::theme::CliTheme;
use crate::{
    file_system::{copy_fs_objects, create_empty_directory},
    templates::{config::RegisteredTemplate, get_templates},
};

pub fn run_new_project_cli_args(options: &CliParserOptions) {
    let project_cmd = options.matches.subcommand_matches("project").unwrap();

    let project_name = if let Some(name) = project_cmd.get_one::<String>("name") {
        name
    } else {
        let input: String = Input::with_theme(&CliTheme::default())
            .with_prompt("Enter the project's name (Required)")
            .with_post_completion_text("Project Name")
            .allow_empty(false)
            .interact()
            .unwrap();

        &input.to_string()
    };

    let supplied_project_path = if let Some(output) = project_cmd.get_one::<String>("output") {
        output
    } else {
        let input: String = Input::with_theme(&CliTheme::default())
            .with_prompt("Enter the project's output path")
            .with_post_completion_text("Project Path")
            .show_default(true)
            .default("./".to_string())
            .interact()
            .unwrap();

        &input.to_string()
    };

    let project_path =
        if PathBuf::from(supplied_project_path).ends_with(format!("/{}", project_name)) {
            supplied_project_path.clone()
        } else {
            format!("{}/{}", supplied_project_path.clone(), project_name)
        };

    let registered_templates = get_templates(options.metadata.templates_meta);
    let template_names = {
        let mut names = Vec::new();
        registered_templates.iter().for_each(|entry| {
            names.push(entry.name.clone());
        });

        names
    };

    if let Some(template) = project_cmd.get_one::<String>("template") {
        let contains_template = template_names.contains(template);

        let template_path = if contains_template {
            format!("{}/{}", options.metadata.templates_meta.directory, template)
        } else {
            template.to_string()
        };

        if template_path == "none" {
            let _ = create_empty_directory(&project_path);
        } else if !contains_template || fs::metadata(&template_path).is_err() {
            // Path does not exist and template does not exist in templates directory, so print an error and exit
            eprintln!(
                "The path {} does not exist! Please supply a valid folder path or an a supported template.",
                template
            );
            process::exit(1);
        } else {
            let template_config = RegisteredTemplate::load_config(&template_path);

            // Copy the files if the template already exists in the templates source folder
            let copy_result =
                copy_fs_objects(template_path, &project_path, &template_config.exclude_paths);
            if copy_result.is_err() {
                eprintln!("Something bad happened while creating the project.",);
                process::exit(1);
            } else {
                let bar = ProgressBar::new_spinner().with_message(format!(
                    "{} {}",
                    style(Emoji("⚙️", "⚙")).blue().bright(),
                    style("Running scripts...").bold()
                ));

                bar.enable_steady_tick(Duration::from_millis(100));

                let project_scripts = template_config.scripts.clone();
                let args = project_scripts
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<&str>>();

                let scripts_output = if cfg!(target_os = "windows") {
                    Command::new("cmd")
                        .current_dir(&project_path)
                        .args(vec![vec!["/C"], args].concat())
                        .output()
                        .expect("Failed to run scripts")
                } else {
                    Command::new("sh")
                        .current_dir(&project_path)
                        .args(vec![vec!["-c"], args].concat())
                        .output()
                        .expect("Failed to run scripts")
                };

                bar.finish_and_clear();

                let _ = io::stdout().write(&scripts_output.stdout);
                let _ = io::stderr().write(&scripts_output.stderr);

                if scripts_output.status.success() {
                    println!(
                        "{} {}",
                        style(Emoji("✅", "✔")).green().bright(),
                        style("Scripts completed successfully!").green().bold()
                    );
                } else {
                    println!(
                        "{} {}",
                        style(Emoji("❌", "𝗑")).red().bright(),
                        style("Scripts completed with errors.").red().bold()
                    );
                }
            }
        }
    } else {
        // Show multiselect prompts
        let templates = template_names;

        if templates.len() > 0 {
            let selection = Select::with_theme(&CliTheme::default())
                .with_prompt("Select project template")
                .default(0)
                .items(&templates)
                .interact()
                .unwrap();

            if selection == 0 {
                let _ = create_empty_directory(&project_path);
            } else {
                let template_source = path::absolute(registered_templates[selection].path.clone())
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();

                let template_config = RegisteredTemplate::load_config(&template_source);

                // Copy the files if the template already exists in the templates source folder
                let copy_result = copy_fs_objects(
                    &template_source,
                    &project_path,
                    &template_config.exclude_paths,
                );

                if copy_result.is_err() {
                    eprintln!("Something bad happened while creating the project.",);
                    process::exit(1);
                } else if template_config.scripts.len() > 0 {
                    let bar = ProgressBar::new_spinner().with_message(format!(
                        "{} {}",
                        style(Emoji("⚙️", "⚙")).blue().bright(),
                        style("Running scripts...").bold()
                    ));

                    bar.enable_steady_tick(Duration::from_millis(100));

                    let project_scripts = template_config.scripts.clone();
                    let args = project_scripts
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<&str>>();

                    let scripts_output = if cfg!(target_os = "windows") {
                        Command::new("cmd")
                            .current_dir(&project_path)
                            .args(vec![vec!["/C"], args].concat())
                            // .stdout(Stdio::piped())
                            // .stderr(Stdio::piped())
                            .output()
                            .expect("Failed to run scripts")
                    } else {
                        Command::new("sh")
                            .current_dir(&project_path)
                            .args(vec![vec!["-c"], args].concat())
                            // .stdout(Stdio::piped())
                            // .stderr(Stdio::piped())
                            .output()
                            .expect("Failed to run scripts")
                    };

                    bar.finish_and_clear();

                    let _ = io::stdout().write(&scripts_output.stdout);
                    let _ = io::stderr().write(&scripts_output.stderr);

                    if scripts_output.status.success() {
                        println!(
                            "{} {}",
                            style(Emoji("✅", "✔")).green().bright(),
                            style("Scripts completed successfully!").green().bold()
                        );
                    } else {
                        println!(
                            "{} {}",
                            style(Emoji("❌", "𝗑")).red().bright(),
                            style("Scripts completed with errors.").red().bold()
                        );
                    }
                } else {
                    println!(
                        "\n{} {}",
                        style(Emoji("✅", "✔")).green().bright(),
                        style("No scripts to run.").yellow().bold(),
                    );
                }
            }
        } else {
            // No templates exist, so print an error and exit
            eprintln!(
                "No templates currently exist so supply a path for the `--template` argument."
            );
            process::exit(1);
        }
    }

    println!(
        "\n{} {}",
        style(Emoji("🚀", "✔")).green().bright(),
        style("All the best!").yellow().bold(),
    );
}
