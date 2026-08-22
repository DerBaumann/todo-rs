use clap::Parser;
use todo_rs::{
    app::App,
    cli::{Cli, Command},
    store::JsonStore,
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut app = App::new(JsonStore::read(cli.store_path)?);

    match cli.command {
        Command::List => app.list()?,
        Command::Get { id } => app.get(id)?,
        Command::Add { title } => app.add(title)?,
        Command::Edit {
            id,
            title,
            completed,
        } => app.edit(id, title, completed)?,
        Command::Complete { id } => app.complete(id)?,
        Command::Delete { id } => app.delete(id)?,
    }

    Ok(())
}
