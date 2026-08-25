use std::io;

use crate::{
    models::{JsonDataError, TodoError},
    store::{JsonStore, JsonStoreError},
};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    JsonStoreError(#[from] JsonStoreError),
    #[error("todo not found")]
    TodoNotFound,
    #[error(transparent)]
    TodoError(#[from] TodoError),
    #[error(transparent)]
    JsonDataError(#[from] JsonDataError),
    #[error(transparent)]
    IOError(#[from] io::Error),
}

type Result<T> = std::result::Result<T, AppError>;

pub struct App {
    store: JsonStore,
    writer: Box<dyn io::Write>,
}

impl App {
    pub fn new(store: JsonStore, writer: Box<dyn io::Write>) -> Self {
        Self { store, writer }
    }

    pub fn list(&self) -> Result<()> {
        for todo in &self.store.data.todos {
            println!("{todo}");
        }

        Ok(())
    }

    pub fn get(&mut self, id: u32) -> Result<()> {
        let todo = self
            .store
            .data
            .find_todo_by_id(id)
            .ok_or(AppError::TodoNotFound)?;

        writeln!(self.writer, "{todo}")?;

        Ok(())
    }

    pub fn add(&mut self, title: String) -> Result<()> {
        self.store.data.add_todo(title.clone())?;

        self.store.write()?;
        println!("Saved todo: {}!", title);

        Ok(())
    }

    pub fn edit(&mut self, id: u32, title: Option<String>, completed: Option<bool>) -> Result<()> {
        self.store.data.edit_todo(id, title, completed)?;

        self.store.write()?;
        println!("Todo updated successfully!");

        Ok(())
    }

    pub fn complete(&mut self, id: u32) -> Result<()> {
        self.store
            .data
            .complete_todo(id)
            .ok_or(AppError::TodoNotFound)?;

        self.store.write()?;
        println!("Todo completed successfully!");

        Ok(())
    }

    pub fn delete(&mut self, id: u32) -> Result<()> {
        self.store
            .data
            .delete_todo(id)
            .ok_or(AppError::TodoNotFound)?;

        self.store.write()?;

        println!("Todo deleted successfully!");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // TODO: Tests
}
