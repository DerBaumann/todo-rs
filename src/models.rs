use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum TodoError {
    #[error("title must be between 4 and 200 characters long")]
    TitleInvalidLength(usize),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn try_new(id: u32, title: String, completed: bool) -> Result<Self, TodoError> {
        let title_char_count = title.chars().count();
        if !(4..=200).contains(&title_char_count) {
            Err(TodoError::TitleInvalidLength(title_char_count))
        } else {
            Ok(Self {
                id,
                title,
                completed,
            })
        }
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let checkmark = if self.completed { "x" } else { " " };
        write!(f, "{} [{checkmark}] {}", self.id, self.title)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JsonData {
    pub todos: Vec<Todo>,
}

// TODO: edit
// TODO: delete
// TODO: complete
impl JsonData {
    pub fn find_todo_by_id(&self, id: u32) -> Option<&Todo> {
        self.todos.iter().find(|t| t.id == id)
    }

    pub fn add_todo(&mut self, title: String) -> Result<&Todo, TodoError> {
        let id = self.todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;

        Ok(self.todos.push_mut(Todo::try_new(id, title, false)?))
    }

    pub fn complete_todo(&mut self, id: u32) -> Option<&Todo> {
        let todo = self.todos.iter_mut().find(|t| t.id == id)?;
        todo.completed = true;
        Some(todo)
    }
}

#[cfg(test)]
mod todo_tests {
    use super::*;

    #[test]
    fn try_new_valid_boundaries_and_unicode() {
        // Lower boundary (exactly 4 chars)
        let min_todo = Todo::try_new(1, "1234".to_string(), false);
        assert_eq!(
            min_todo,
            Ok(Todo {
                id: 1,
                title: "1234".to_string(),
                completed: false,
            })
        );

        // Upper boundary (exactly 200 chars)
        let max_title = "a".repeat(200);
        let max_todo = Todo::try_new(2, max_title.clone(), true);
        assert_eq!(
            max_todo,
            Ok(Todo {
                id: 2,
                title: max_title,
                completed: true,
            })
        );

        // Unicode character count check (4 multi-byte characters)
        let unicode_todo = Todo::try_new(3, "🦀🦀🦀🦀".to_string(), false);
        assert!(unicode_todo.is_ok());
    }

    #[test]
    fn try_new_invalid_lengths() {
        // Empty title (0 chars)
        assert_eq!(
            Todo::try_new(1, "".to_string(), false),
            Err(TodoError::TitleInvalidLength(0))
        );

        // Just below lower boundary (3 chars)
        assert_eq!(
            Todo::try_new(2, "123".to_string(), false),
            Err(TodoError::TitleInvalidLength(3))
        );

        // Just above upper boundary (201 chars)
        let too_long_title = "a".repeat(201);
        assert_eq!(
            Todo::try_new(3, too_long_title, false),
            Err(TodoError::TitleInvalidLength(201))
        );
    }

    #[test]
    fn display_uncompleted_todo() {
        let todo = Todo {
            id: 1,
            title: "Buy groceries".to_string(),
            completed: false,
        };
        assert_eq!(todo.to_string(), "1 [ ] Buy groceries");
    }

    #[test]
    fn display_completed_todo() {
        let todo = Todo {
            id: 42,
            title: "Write Rust tests".to_string(),
            completed: true,
        };
        assert_eq!(todo.to_string(), "42 [x] Write Rust tests");
    }
}

#[cfg(test)]
mod json_data {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn add_todo_auto_increments_id() -> anyhow::Result<()> {
        let mut data = JsonData::default();

        let first = data.add_todo("first".to_string())?;
        assert_eq!(first.id, 1);
        assert_eq!(first.title, "first");
        assert!(!first.completed);

        let second = data.add_todo("second".to_string())?;
        assert_eq!(second.id, 2);
        assert_eq!(second.title, "second");
        assert_eq!(data.todos.len(), 2);

        Ok(())
    }

    #[test]
    fn find_todo_by_id_found() -> anyhow::Result<()> {
        let mut data = JsonData::default();

        data.add_todo("Buy milk".to_string())?;
        data.add_todo("Walk dog".to_string())?;

        let todo = data.find_todo_by_id(2);
        assert!(todo.is_some());

        let todo = todo.unwrap();

        assert_eq!(todo.id, 2);
        assert_eq!(todo.title, "Walk dog");

        Ok(())
    }

    #[test]
    fn find_todo_by_id_not_found() -> anyhow::Result<()> {
        let mut data = JsonData::default();

        data.add_todo("Buy milk".to_string())?;
        data.add_todo("Walk dog".to_string())?;

        assert_eq!(data.find_todo_by_id(999), None);
        assert_eq!(JsonData::default().find_todo_by_id(1), None);

        Ok(())
    }

    #[test]
    fn complete_existing_todo() -> anyhow::Result<()> {
        let mut data = JsonData::default();

        let id = {
            let todo = data.add_todo("Cook lunch".to_string())?;
            todo.id
        };

        let todo = data.complete_todo(id);

        assert!(todo.is_some());

        let todo = todo.unwrap();

        assert!(todo.completed);

        Ok(())
    }

    #[test]
    fn complete_nonexisting_todo() {
        let mut data = JsonData::default();

        assert_eq!(data.complete_todo(420), None);
    }
}
