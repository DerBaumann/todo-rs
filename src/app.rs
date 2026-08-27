use std::io::{self, Write};

use crate::{
    models::{TodoError, TodoList, TodoListError},
    store::{DataStore, JsonStoreError},
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
    JsonDataError(#[from] TodoListError),
    #[error(transparent)]
    IOError(#[from] io::Error),
}

type Result<T> = std::result::Result<T, AppError>;

pub struct App<S, W> {
    store: S,
    writer: W,
}

impl<S, W> App<S, W>
where
    S: DataStore<Data = TodoList, Error = JsonStoreError>,
    W: Write,
{
    pub fn new(store: S, writer: W) -> Self {
        Self { store, writer }
    }

    pub fn list(&mut self) -> Result<()> {
        for todo in &self.store.data().todos {
            writeln!(self.writer, "{todo}")?;
        }

        Ok(())
    }

    pub fn get(&mut self, id: u32) -> Result<()> {
        let todo = self
            .store
            .data()
            .find_todo_by_id(id)
            .ok_or(AppError::TodoNotFound)?;

        writeln!(self.writer, "{todo}")?;

        Ok(())
    }

    pub fn add(&mut self, title: String) -> Result<()> {
        self.store.data_mut().add_todo(title.clone())?;

        self.store.write()?;
        writeln!(self.writer, "Saved todo: {}!", title)?;

        Ok(())
    }

    pub fn edit(&mut self, id: u32, title: Option<String>, completed: Option<bool>) -> Result<()> {
        self.store.data_mut().edit_todo(id, title, completed)?;

        self.store.write()?;
        writeln!(self.writer, "Todo updated successfully!")?;

        Ok(())
    }

    pub fn complete(&mut self, id: u32) -> Result<()> {
        self.store
            .data_mut()
            .complete_todo(id)
            .ok_or(AppError::TodoNotFound)?;

        self.store.write()?;
        writeln!(self.writer, "Todo completed successfully!")?;

        Ok(())
    }

    pub fn delete(&mut self, id: u32) -> Result<()> {
        self.store
            .data_mut()
            .delete_todo(id)
            .ok_or(AppError::TodoNotFound)?;

        self.store.write()?;

        writeln!(self.writer, "Todo deleted successfully!")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // TODO: Tests
}
