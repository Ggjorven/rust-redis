use std::{sync::{Arc, Mutex}, collections::HashMap};

use super::super::resp::*;

//////////////////////////////////////////
// DatabaseError
//////////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseError
{
    DoesntExist(String),
    NotImplemented
}

//////////////////////////////////////////
// Database
//////////////////////////////////////////
pub struct Database 
{
    inner: Arc<Mutex<HashMap<String, DataType>>>,
}

impl Database
{
    pub fn new() -> Self
    {
        Database { 
            inner: Arc::new(Mutex::new(HashMap::new())) 
        }
    }

    pub fn set(&mut self, name: &str, value: &DataType) -> Result<(), DatabaseError>
    {
        let inner = Arc::clone(&self.inner);
        let mut db = inner.lock().unwrap();

        db.entry(name.to_string())
            .and_modify(|v| { *v = value.clone(); })
            .or_insert(value.clone());

        Ok(())
    }

    pub fn get(&self, name: &str) -> Result<DataType, DatabaseError>
    {
        let inner = Arc::clone(&self.inner);
        let db = inner.lock().unwrap();

        let entry = db.get(name);

        match entry
        {
            Some(data_type) => { Ok(data_type.clone()) }
            None => { Err(DatabaseError::DoesntExist(format!("Entry by name: {} doesn't exist.", name))) }
        }
    }
}