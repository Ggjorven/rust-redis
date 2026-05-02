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
fn raw_ping_command() -> Result<(), CommandError> {
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
fn parsed_ping_command() -> Result<(), CommandError> {
    let mut db = Database::new();

    let Ok(command1_result) = parse_data_type("+PING\r\n", &mut 0) else { return Err(CommandError::NonRunnableCommand); };
    assert_eq!(run_command(&mut db, command1_result)?, DataType::SimpleString("OK".to_string()));

    let Ok(command2_result) = parse_data_type("$4\r\nPING\r\n", &mut 0) else { return Err(CommandError::NonRunnableCommand); };
    assert_eq!(run_command(&mut db, command2_result)?, DataType::SimpleString("OK".to_string()));

    let Ok(command3_result) = parse_data_type("*2\r\n+PING\r\n+HELLO WORLD\r\n", &mut 0) else { return Err(CommandError::NonRunnableCommand); };
    assert_eq!(run_command(&mut db, command3_result)?, DataType::SimpleString("HELLO WORLD".to_string()));

    let Ok(command4_result) = parse_data_type("*2\r\n+PING\r\n$22\r\nSECOND TIME USING RUST\r\n", &mut 0) else { return Err(CommandError::NonRunnableCommand); };
    assert_eq!(run_command(&mut db, command4_result)?, DataType::SimpleString("SECOND TIME USING RUST".to_string()));

    let Ok(command5_result) = parse_data_type("*2\r\n$4\r\nPING\r\n$2\r\nHI\r\n", &mut 0) else { return Err(CommandError::NonRunnableCommand); };
    assert_eq!(run_command(&mut db, command5_result)?, DataType::SimpleString("HI".to_string()));

    Ok(())
}

#[test]
fn raw_set_command() -> Result<(), CommandError> {
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

#[test]
fn parsed_set_command() -> Result<(), CommandError> {
    let mut db = Database::new();

    let Ok(command1_result) = parse_data_type("*3\r\n+SET\r\n+foo\r\n+bar\r\n", &mut 0) else { return Err(CommandError::NonRunnableCommand); };
    assert_eq!(run_command(&mut db, command1_result)?, DataType::SimpleString("OK".to_string()));

    let Ok(command2_result) = parse_data_type("*3\r\n+SET\r\n+bar\r\n,1.11\r\n", &mut 0) else { return Err(CommandError::NonRunnableCommand); };
    assert_eq!(run_command(&mut db, command2_result)?, DataType::SimpleString("OK".to_string()));

    Ok(())
}

#[test]
fn raw_get_command() -> Result<(), CommandError> {
    let mut db = Database::new();

    let _ = db.set("foo", &DataType::Integer(420));
    let _ = db.set("bar", &DataType::SimpleString("hello world".to_string()));

    assert_eq!(run_command(&mut db, DataType::Array(vec![
        DataType::SimpleString("GET".to_string()),
        DataType::SimpleString("foo".to_string())
    ]))?, DataType::Integer(420));

    assert_eq!(run_command(&mut db, DataType::Array(vec![
        DataType::SimpleString("GET".to_string()),
        DataType::SimpleString("bar".to_string())
    ]))?, DataType::SimpleString("hello world".to_string()));

    Ok(())
}

#[test]
fn parsed_get_command() -> Result<(), CommandError> {
    let mut db = Database::new();

    let _ = db.set("foo", &DataType::Integer(420));
    let _ = db.set("bar", &DataType::SimpleString("hello world".to_string()));

    let Ok(command1_result) = parse_data_type("*2\r\n+GET\r\n+foo\r\n", &mut 0) else { return Err(CommandError::NonRunnableCommand); };
    assert_eq!(run_command(&mut db, command1_result)?, DataType::Integer(420));

    let Ok(command2_result) = parse_data_type("*2\r\n+GET\r\n+bar\r\n", &mut 0) else { return Err(CommandError::NonRunnableCommand); };
    assert_eq!(run_command(&mut db, command2_result)?, DataType::SimpleString("hello world".to_string()));

    Ok(())
}


#[test]
fn raw_del_command() -> Result<(), CommandError> {
    let mut db = Database::new();

    let _ = db.set("foo", &DataType::Integer(420));
    let _ = db.set("bar", &DataType::SimpleString("hello world".to_string()));

    assert_eq!(run_command(&mut db, DataType::Array(vec![
        DataType::SimpleString("DEL".to_string()),
        DataType::SimpleString("foo".to_string())
    ]))?, DataType::SimpleString("OK".to_string()));

    assert_eq!(run_command(&mut db, DataType::Array(vec![
        DataType::SimpleString("DEL".to_string()),
        DataType::SimpleString("bar".to_string())
    ]))?, DataType::SimpleString("OK".to_string()));

    Ok(())
}

#[test]
fn parsed_del_command() -> Result<(), CommandError> {
    let mut db = Database::new();

    let _ = db.set("foo", &DataType::Integer(420));
    let _ = db.set("bar", &DataType::SimpleString("hello world".to_string()));

    let Ok(command1_result) = parse_data_type("*2\r\n+DEL\r\n+foo\r\n", &mut 0) else { return Err(CommandError::NonRunnableCommand); };
    assert_eq!(run_command(&mut db, command1_result)?, DataType::SimpleString("OK".to_string()));

    let Ok(command2_result) = parse_data_type("*2\r\n+DEL\r\n+bar\r\n", &mut 0) else { return Err(CommandError::NonRunnableCommand); };
    assert_eq!(run_command(&mut db, command2_result)?, DataType::SimpleString("OK".to_string()));

    Ok(())
}