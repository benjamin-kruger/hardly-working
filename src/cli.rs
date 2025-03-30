use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "hw", about = "Working hard or hardly working", version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Config {
        #[clap(short, long, value_parser)]
        file_path: Option<String>,

        #[clap(long, action)]
        show: bool,
    },

    Add {
        #[clap(short, long, value_parser)]
        group: Option<String>,

        #[clap(value_parser)]
        description: Vec<String>,
    },

    #[clap(alias = "ls")]
    List,

    Toggle {
        #[clap(value_parser)]
        line: usize,
    },

    Remove {
        #[clap(value_parser)]
        line: usize,
    },
}
