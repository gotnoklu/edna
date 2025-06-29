use std::fs;

use config::{RegisteredTemplate, TemplatesConfig, TemplatesMetadata};

pub mod config;

pub fn get_templates(metadata: &TemplatesMetadata) -> Vec<RegisteredTemplate> {
    let config_details = TemplatesConfig::load(&metadata);

    let mut registered_templates: Vec<RegisteredTemplate> = Vec::new();
    registered_templates.push(RegisteredTemplate {
        name: String::from("(No template)"),
        path: String::from(""),
    });

    for entry in config_details.registry {
        let metadata = fs::metadata(&entry.path).unwrap();
        if metadata.is_dir() {
            registered_templates.push(entry);
        }
    }

    registered_templates
}
