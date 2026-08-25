use std::{fs, io, path::PathBuf};

use crate::models::JsonData;

#[derive(Debug, thiserror::Error)]
pub enum JsonStoreError {
    #[error(transparent)]
    IOError(#[from] io::Error),

    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}

#[mockall::automock(type Data=JsonData; type Error=JsonStoreError;)]
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

pub struct JsonStore {
    pub store_path: PathBuf,
    pub data: JsonData,
}

impl DataStore for JsonStore {
    type Data = JsonData;
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
                let empty = JsonData::default();
                let json = serde_json::to_string_pretty(&empty)?;
                fs::write(&path, &json)?;
                Ok(json)
            }
            _ => Err(e),
        })?;

        Ok(Self {
            store_path: path,
            data: serde_json::from_str::<JsonData>(&contents)?,
        })
    }

    fn write(&self) -> Result<(), Self::Error> {
        let contents = serde_json::to_string_pretty(&self.data)?;
        fs::write(&self.store_path, contents)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // TODO: Tests
}
