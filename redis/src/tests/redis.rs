use super::super::resp::*;
use super::super::redis::*;

//////////////////////////////////////////
// Database
//////////////////////////////////////////
#[test]
fn database_queries() -> Result<(), DatabaseError> {
    let mut db = Database::new();

    let name = "foo";
    let value = DataType::SimpleString("bar".to_string());
    
    db.set(name, &value)?;
    assert_eq!(db.get(name)?, DataType::SimpleString("bar".to_string()));

    db.del(name)?;
    assert!(!db.exists(name));

    Ok(())
}

//////////////////////////////////////////
// Commands
//////////////////////////////////////////
#[test]
fn ping_command() -> Result<(), CommandError> {
    let mut db = Database::new();

    assert_eq!(run_command(&mut db, DataType::SimpleString("PING".to_string()))?, DataType::SimpleString("OK".to_string()));
    assert_eq!(run_command(&mut db, DataType::BulkString(Some("PING".to_string())))?, DataType::SimpleString("OK".to_string()));

    assert_eq!(run_command(&mut db, DataType::Array(vec![
        DataType::SimpleString("PING".to_string()),
        DataType::SimpleString("HELLO WORLD".to_string()),
    ]))?, DataType::SimpleString("HELLO WORLD".to_string()));

    assert_eq!(run_command(&mut db, DataType::Array(vec![
        DataType::SimpleString("PING".to_string()),
        DataType::BulkString(Some("SECOND TIME USING RUST".to_string())),
    ]))?, DataType::SimpleString("SECOND TIME USING RUST".to_string()));

    assert_eq!(run_command(&mut db, DataType::Array(vec![
        DataType::BulkString(Some("PING".to_string())),
        DataType::BulkString(Some("HI".to_string())),
    ]))?, DataType::SimpleString("HI".to_string()));

    Ok(())
}

#[test]
fn set_command() -> Result<(), CommandError> {
    let mut db = Database::new();

    assert_eq!(run_command(&mut db, DataType::Array(vec![
        DataType::SimpleString("SET".to_string()),
        DataType::SimpleString("foo".to_string()),
        DataType::SimpleString("bar".to_string())
    ]))?, DataType::SimpleString("OK".to_string()));

    assert_eq!(run_command(&mut db, DataType::Array(vec![
        DataType::SimpleString("SET".to_string()),
        DataType::SimpleString("bar".to_string()),
        DataType::Double(1.11)
    ]))?, DataType::SimpleString("OK".to_string()));

    Ok(())
}