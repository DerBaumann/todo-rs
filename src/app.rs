use crate::{
    models::TodoError,
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
}

type Result<T> = std::result::Result<T, AppError>;

pub struct App {
    store: JsonStore,
}

impl App {
    pub fn new(store: JsonStore) -> Self {
        Self { store }
    }

    pub fn list(&self) -> Result<()> {
        for todo in &self.store.data.todos {
            println!("{todo}");
        }

        Ok(())
    }

    pub fn get(&self, id: u32) -> Result<()> {
        let todo = self
            .store
            .data
            .find_todo_by_id(id)
            .ok_or(AppError::TodoNotFound)?;

        println!("{todo}");

        Ok(())
    }

    pub fn add(&mut self, title: String) -> Result<()> {
        self.store.data.add_todo(title.clone())?;

        self.store.write()?;
        println!("Saved todo: {}!", title);

        Ok(())
    }

    pub fn edit(&mut self, id: u32, title: Option<String>, completed: Option<bool>) -> Result<()> {
        let todo = self
            .store
            .data
            .todos
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(AppError::TodoNotFound)?;

        todo.title = title.unwrap_or_else(|| todo.title.to_owned());
        todo.completed = completed.unwrap_or(todo.completed);

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
