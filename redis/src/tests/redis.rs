use super::super::resp;
use super::super::redis;

//////////////////////////////////////////
// Database
//////////////////////////////////////////
#[test]
fn database_queries() -> Result<(), redis::DatabaseError> {
    let mut db = redis::Database::new();

    let name = "foo";
    let value = resp::DataType::SimpleString("bar".to_string());
    db.set(name, &value)?;

    assert_eq!(db.get(name)?, resp::DataType::SimpleString("bar".to_string()));

    Ok(())
}

//////////////////////////////////////////
// Commands
//////////////////////////////////////////
#[test]
fn ping_command() -> Result<(), redis::CommandError> {
    let mut db = redis::Database::new();

    assert_eq!(redis::run_command(&mut db, resp::DataType::SimpleString("PING".to_string()))?, resp::DataType::SimpleString("OK".to_string()));
    assert_eq!(redis::run_command(&mut db, resp::DataType::BulkString(Some("PING".to_string())))?, resp::DataType::SimpleString("OK".to_string()));

    assert_eq!(redis::run_command(&mut db, resp::DataType::Array(vec![
        resp::DataType::SimpleString("PING".to_string()),
        resp::DataType::SimpleString("HELLO WORLD".to_string()),
    ]))?, resp::DataType::SimpleString("HELLO WORLD".to_string()));

    assert_eq!(redis::run_command(&mut db, resp::DataType::Array(vec![
        resp::DataType::SimpleString("PING".to_string()),
        resp::DataType::BulkString(Some("SECOND TIME USING RUST".to_string())),
    ]))?, resp::DataType::SimpleString("SECOND TIME USING RUST".to_string()));

    assert_eq!(redis::run_command(&mut db, resp::DataType::Array(vec![
        resp::DataType::BulkString(Some("PING".to_string())),
        resp::DataType::BulkString(Some("HI".to_string())),
    ]))?, resp::DataType::SimpleString("HI".to_string()));

    Ok(())
}