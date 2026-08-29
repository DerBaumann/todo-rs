use std::io::{self, Write};

use crate::{
    models::{TodoError, TodoList, TodoListError},
    store::{DataStore, JsonStoreError},
};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    // TODO: Remove this
    #[error("todo not found")]
    TodoNotFound,
    #[error(transparent)]
    JsonStoreError(#[from] JsonStoreError),
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

// TODO: Test App::delete
#[cfg(test)]
mod app_tests {
    use crate::{models::Todo, store::MockDataStore};

    use super::*;

    #[test]
    fn list_empty_store() -> anyhow::Result<()> {
        let mut mock_store = MockDataStore::new();
        mock_store
            .expect_data()
            .times(1)
            .return_const(TodoList { todos: Vec::new() });

        let mut app = App::new(mock_store, Vec::new());

        app.list()?;

        assert!(app.writer.is_empty());

        Ok(())
    }

    #[test]
    fn list_nonempty_store() -> anyhow::Result<()> {
        let mut mock_store = MockDataStore::new();
        mock_store.expect_data().times(1).return_const(TodoList {
            todos: vec![
                Todo::try_new(1, "test".to_string(), false)?,
                Todo::try_new(2, "test 2".to_string(), true)?,
            ],
        });

        let mut app = App::new(mock_store, Vec::new());

        app.list()?;

        assert_eq!(String::from_utf8(app.writer)?, "1 [ ] test\n2 [x] test 2\n");

        Ok(())
    }

    #[test]
    fn get_existing_todo() -> anyhow::Result<()> {
        let mut mock_store = MockDataStore::new();
        mock_store.expect_data().times(1).return_const(TodoList {
            todos: vec![Todo::try_new(1, "test".to_string(), false)?],
        });

        let mut app = App::new(mock_store, Vec::new());

        app.get(1)?;

        assert_eq!(String::from_utf8(app.writer)?, "1 [ ] test\n");

        Ok(())
    }

    #[test]
    fn get_nonexisting_todo() -> anyhow::Result<()> {
        let mut mock_store = MockDataStore::new();
        mock_store
            .expect_data()
            .times(1)
            .return_const(TodoList { todos: Vec::new() });

        let mut app = App::new(mock_store, Vec::new());

        assert!(matches!(app.get(420), Err(AppError::TodoNotFound)));

        Ok(())
    }

    #[test]
    fn add_todo() -> anyhow::Result<()> {
        let mut mock_store = MockDataStore::new();
        mock_store
            .expect_data_mut()
            .times(1)
            .returning(|| TodoList { todos: Vec::new() });

        mock_store.expect_write().times(1).returning(|| Ok(()));

        let mut app = App::new(mock_store, Vec::new());

        app.add("Test Todo".to_string())?;

        assert_eq!(String::from_utf8(app.writer)?, "Saved todo: Test Todo!\n");

        Ok(())
    }

    #[test]
    fn edit_todo() -> anyhow::Result<()> {
        let mut mock_store = MockDataStore::new();
        mock_store
            .expect_data_mut()
            .times(1)
            .returning(|| TodoList {
                todos: vec![
                    Todo::try_new(1, "test".to_string(), false).expect("Todo should be valid"),
                ],
            });

        mock_store.expect_write().times(1).returning(|| Ok(()));

        let mut app = App::new(mock_store, Vec::new());

        app.edit(1, Some("edited todo".to_string()), None)?;

        assert_eq!(
            String::from_utf8(app.writer)?,
            "Todo updated successfully!\n"
        );

        Ok(())
    }

    #[test]
    fn complete_todo() -> anyhow::Result<()> {
        let mut mock_store = MockDataStore::new();
        mock_store
            .expect_data_mut()
            .times(1)
            .returning(|| TodoList {
                todos: vec![
                    Todo::try_new(1, "test".to_string(), false).expect("Todo should be valid"),
                ],
            });

        mock_store.expect_write().times(1).returning(|| Ok(()));

        let mut app = App::new(mock_store, Vec::new());

        app.complete(1)?;

        assert_eq!(
            String::from_utf8(app.writer)?,
            "Todo completed successfully!\n"
        );

        Ok(())
    }

    #[test]
    fn complete_nonexisting_todo() -> anyhow::Result<()> {
        let mut mock_store = MockDataStore::new();
        mock_store
            .expect_data_mut()
            .times(1)
            .returning(|| TodoList { todos: Vec::new() });

        let mut app = App::new(mock_store, Vec::new());

        assert!(matches!(app.complete(420), Err(AppError::TodoNotFound)));

        Ok(())
    }
}
