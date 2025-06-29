use clap::ArgMatches;

use crate::templates::config::TemplatesMetadata;

pub struct CliMetadata<'a> {
    pub templates_meta: &'a TemplatesMetadata,
}

pub struct CliParserOptions<'a> {
    pub metadata: &'a CliMetadata<'a>,
    pub matches: &'a ArgMatches,
}
