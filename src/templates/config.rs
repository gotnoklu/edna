use std::{
    fs,
    io::{self, Write},
    path, process,
};

use serde::{Deserialize, Deserializer, Serialize};

use crate::file_system::copy_fs_objects;

pub struct TemplatesMetadata {
    pub directory: String,
    pub filename: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreatedTemplateConfig {
    pub target: String,
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub author: String,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub version: String,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub description: String,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub exclude_paths: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub scripts: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub exclude_config: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisteredTemplate {
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplatesConfig {
    pub target: String,
    pub registry: Vec<RegisteredTemplate>,
}

fn deserialize_optional_field<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let option_result = Option::deserialize(deserializer)?;
    Ok(option_result.unwrap_or_default())
}

impl TemplatesConfig {
    pub fn get_path(metadata: &TemplatesMetadata) -> String {
        let path_buffer = path::absolute(&metadata.directory)
            .unwrap()
            .join(&metadata.filename);

        path_buffer.to_str().unwrap().to_string()
    }

    pub fn load(metadata: &TemplatesMetadata) -> TemplatesConfig {
        let file = fs::File::open(Self::get_path(&metadata)).unwrap();
        let config: TemplatesConfig = serde_json::from_reader(file).unwrap();
        config
    }

    pub fn register_template(
        template: &RegisteredTemplate,
        metadata: &TemplatesMetadata,
    ) -> Result<(), io::Error> {
        let mut config = Self::load(&metadata);
        let path = Self::get_path(&metadata);
        config.registry.push(template.clone());
        let file_writer = fs::File::create(path).unwrap();
        let _ = serde_json::to_writer(file_writer, &config).unwrap();

        Ok(())
    }
}

impl CreatedTemplateConfig {
    pub fn create_config(path: &String, config: &CreatedTemplateConfig) -> Result<(), io::Error> {
        let file_writer = fs::File::create(path).unwrap();
        let _ = serde_json::to_writer(file_writer, config).unwrap();
        Ok(())
    }

    pub fn create_template(
        output: &String,
        source: &String,
        config: &CreatedTemplateConfig,
        metadata: &TemplatesMetadata,
    ) -> Result<(), io::Error> {
        let template_exists = fs::exists(&output).unwrap();
        if template_exists {
            eprintln!("The template \"{}\" already exists!", &output);
            process::exit(1);
        }

        if source == "" {
            let _ = fs::create_dir(output).unwrap();
        } else {
            let _ = copy_fs_objects(source, output, &config.exclude_paths);
        }

        let _ = Self::create_config(
            &String::from(format!("{}/{}", output, "edna.config.json")),
            config,
        );

        let _ = TemplatesConfig::register_template(
            &RegisteredTemplate {
                name: config.name.clone(),
                path: output.to_string(),
            },
            metadata,
        );

        Ok(())
    }
}

impl RegisteredTemplate {
    pub fn load_config(template_path: &String) -> CreatedTemplateConfig {
        let resolved_path = format!("{}/{}", &template_path, "edna.config.json");
        let config_exists = fs::exists(&resolved_path).unwrap();

        if !config_exists {
            println!("{}, {}", &resolved_path, &template_path);
            let mut file = fs::File::create_new(&resolved_path).unwrap();
            file.write_all(
                format!(
                    r#"{{
  "target": "project",
  "author": "",
  "name": "{}",
  "version": "1.0.0",
  "exclude_config": true,
  "exclude_paths": [],
  "scripts": []
}}
            "#,
                    &template_path.split("/").last().unwrap()
                )
                .as_bytes(),
            )
            .unwrap();
        }

        let file = fs::File::open(&resolved_path).unwrap();
        let mut config: CreatedTemplateConfig = serde_json::from_reader(file).unwrap();

        if config.target != "project" {
            eprintln!(
                "Invalid template config for {}. The variant must be `project`",
                &resolved_path
            );
            process::exit(1);
        }

        if config.exclude_config {
            config.exclude_paths.push(resolved_path);
        }

        config
    }
}
