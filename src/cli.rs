use std::path::PathBuf;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long, env)]
    pub store_path: PathBuf,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Clone)]
pub enum Command {
    List,
    ListCompleted,
    Get {
        id: u32,
    },
    Add {
        title: String,
    },
    Edit {
        id: u32,
        #[arg(short, long)]
        title: Option<String>,
        #[arg(short, long)]
        completed: Option<bool>,
    },
    Complete {
        id: u32,
    },
    Delete {
        id: u32,
    },
}
