use std::{fs, io, path::PathBuf};

#[cfg(test)]
use mockall::{automock, predicate::*};

use crate::models::TodoList;

#[derive(Debug, thiserror::Error)]
pub enum JsonStoreError {
    #[error(transparent)]
    IOError(#[from] io::Error),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}

#[cfg_attr(test, automock(type Data=TodoList; type Error=JsonStoreError;))]
pub trait DataStore {
    type Data;
    type Error;

    fn data(&self) -> &Self::Data;
    fn data_mut(&mut self) -> &mut Self::Data;

    fn read(path: PathBuf) -> Result<Self, Self::Error>
    where
        Self: std::marker::Sized;
    fn write(&self) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub struct JsonStore {
    pub store_path: PathBuf,
    pub data: TodoList,
}

impl DataStore for JsonStore {
    type Data = TodoList;
    type Error = JsonStoreError;

    fn data(&self) -> &Self::Data {
        &self.data
    }

    fn data_mut(&mut self) -> &mut Self::Data {
        &mut self.data
    }

    fn read(path: PathBuf) -> Result<Self, Self::Error>
    where
        Self: std::marker::Sized,
    {
        let contents = fs::read_to_string(&path).or_else(|e| match e.kind() {
            std::io::ErrorKind::NotFound => {
                let empty = TodoList::default();
                let json = serde_json::to_string_pretty(&empty)?;
                fs::write(&path, &json)?;
                Ok(json)
            }
            _ => Err(e),
        })?;

        Ok(Self {
            store_path: path,
            data: serde_json::from_str::<TodoList>(&contents)?,
        })
    }

    fn write(&self) -> Result<(), Self::Error> {
        let contents = serde_json::to_string_pretty(&self.data)?;
        fs::write(&self.store_path, contents)?;
        Ok(())
    }
}

#[cfg(test)]
mod json_store_tests {
    use super::*;
    use crate::models::Todo;
    use assert_fs::TempDir;
    use assert_fs::prelude::*;

    #[test]
    fn test_data_accessors() {
        let mut store = JsonStore {
            store_path: PathBuf::from("dummy.json"),
            data: TodoList::default(),
        };

        assert!(store.data().todos.is_empty());

        store.data_mut().todos.push(Todo {
            id: 1,
            title: "Test item".to_string(),
            completed: false,
        });

        assert_eq!(store.data().todos.len(), 1);
        assert_eq!(store.data().todos[0].title, "Test item");
    }

    #[test]
    fn test_read_creates_default_when_file_missing() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.child("missing_todos.json");

        file_path.assert(predicates::path::missing());

        let store =
            JsonStore::read(file_path.path().to_path_buf()).expect("should create and read store");

        assert_eq!(store.store_path, file_path.path());
        assert!(store.data().todos.is_empty());
        file_path.assert(predicates::path::exists());
    }

    #[test]
    fn test_read_existing_valid_file() {
        let temp_dir = TempDir::new().unwrap();
        let file = temp_dir.child("todos.json");
        file.write_str(
            r#"{
              "todos": [
                {
                  "id": 1,
                  "title": "Buy milk",
                  "completed": false
                }
              ]
            }"#,
        )
        .unwrap();

        let store = JsonStore::read(file.path().to_path_buf()).expect("should read existing file");

        assert_eq!(store.data().todos.len(), 1);
        assert_eq!(store.data().todos[0].id, 1);
        assert_eq!(store.data().todos[0].title, "Buy milk");
        assert!(!store.data().todos[0].completed);
    }

    #[test]
    fn test_read_invalid_json_returns_error() {
        let temp_dir = TempDir::new().unwrap();
        let file = temp_dir.child("corrupt.json");
        file.write_str("invalid json data").unwrap();

        let result = JsonStore::read(file.path().to_path_buf());

        assert!(
            matches!(result, Err(JsonStoreError::JsonError(_))),
            "expected JsonError, got {:?}",
            result
        );
    }

    #[test]
    fn test_read_io_error_for_directory_path() {
        let temp_dir = TempDir::new().unwrap();

        // Reading a directory path as a file triggers an I/O error (not ErrorKind::NotFound)
        let result = JsonStore::read(temp_dir.path().to_path_buf());

        assert!(
            matches!(result, Err(JsonStoreError::IOError(_))),
            "expected IOError, got {:?}",
            result
        );
    }

    #[test]
    fn test_write_persists_data_to_file() {
        let temp_dir = TempDir::new().unwrap();
        let file = temp_dir.child("todos.json");

        let store = JsonStore {
            store_path: file.path().to_path_buf(),
            data: TodoList {
                todos: vec![Todo {
                    id: 42,
                    title: "Write integration tests".to_string(),
                    completed: true,
                }],
            },
        };

        store.write().expect("should write store to disk");

        file.assert(predicates::str::contains(r#""id": 42"#));
        file.assert(predicates::str::contains(
            r#""title": "Write integration tests""#,
        ));
        file.assert(predicates::str::contains(r#""completed": true"#));
    }
}
