use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Json,
    Plain,
    Yaml,
}
