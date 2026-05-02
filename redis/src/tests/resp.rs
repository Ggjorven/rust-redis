use super::super::resp::*;

//////////////////////////////////////////
// Parse tests
//////////////////////////////////////////
#[test]
fn parse_simple_string_type() -> Result<(), ParseError> {
    assert_eq!(parse_data_type("+test\r\n", &mut 0)?, DataType::SimpleString("test".to_string()));
    assert_eq!(parse_data_type("+test message hihi\r\n", &mut 0)?, DataType::SimpleString("test message hihi".to_string()));

    Ok(())
}

#[test]
fn parse_simple_error_type() -> Result<(), ParseError> {
    assert_eq!(parse_data_type("-test\r\n", &mut 0)?, DataType::SimpleError("test".to_string()));
    assert_eq!(parse_data_type("-test message hihi\r\n", &mut 0)?, DataType::SimpleError("test message hihi".to_string()));

    Ok(())
}

#[test]
fn parse_integer_type() -> Result<(), ParseError> {
    assert_eq!(parse_data_type(":-1\r\n", &mut 0)?, DataType::Integer(-1));
    assert_eq!(parse_data_type(":0\r\n", &mut 0)?, DataType::Integer(0));
    assert_eq!(parse_data_type(":100\r\n", &mut 0)?, DataType::Integer(100));

    Ok(())
}

#[test]
fn parse_bulk_string_type() -> Result<(), ParseError> {
    assert_eq!(parse_data_type("$-1\r\n", &mut 0)?, DataType::BulkString(None));
    assert_eq!(parse_data_type("$5\r\nhello\r\n", &mut 0)?, DataType::BulkString(Some("hello".to_string())));
    assert_eq!(parse_data_type("$10\r\nhello\r\nyou\r\n", &mut 0)?, DataType::BulkString(Some("hello\r\nyou".to_string())));

    Ok(())
}

#[test]
fn parse_array_type() -> Result<(), ParseError> {
    assert_eq!(parse_data_type("*0\r\n", &mut 0)?, DataType::Array(Vec::new()));
    assert_eq!(parse_data_type("*2\r\n$5\r\nhello\r\n$5\r\nworld\r\n", &mut 0)?, DataType::Array(vec![
        DataType::BulkString(Some("hello".to_string())), 
        DataType::BulkString(Some("world".to_string()))
    ]));
    assert_eq!(parse_data_type("*3\r\n:1\r\n:2\r\n:3\r\n", &mut 0)?, DataType::Array(vec![
        DataType::Integer(1), 
        DataType::Integer(2), 
        DataType::Integer(3)
    ]));

    assert_eq!(parse_data_type("*5\r\n:1\r\n:2\r\n:3\r\n:4\r\n$5\r\nhello\r\n", &mut 0)?, DataType::Array(vec![
        DataType::Integer(1), 
        DataType::Integer(2), 
        DataType::Integer(3),
        DataType::Integer(4),
        DataType::BulkString(Some("hello".to_string()))
    ]));

    assert_eq!(parse_data_type("*2\r\n*3\r\n:1\r\n:2\r\n:3\r\n*2\r\n+Hello\r\n-World\r\n", &mut 0)?, DataType::Array(vec![
        DataType::Array(vec![
            DataType::Integer(1), 
            DataType::Integer(2),
            DataType::Integer(3)
        ]),
        DataType::Array(vec![
            DataType::SimpleString("Hello".to_string()), 
            DataType::SimpleError("World".to_string()), 
        ]),
    ]));

    // TODO: More when new data types are added

    Ok(())
}

#[test]
fn parse_null_type() -> Result<(), ParseError> {
    assert_eq!(parse_data_type("_\r\n", &mut 0)?, DataType::Null);

    Ok(())
}

#[test]
fn parse_boolean_type() -> Result<(), ParseError> {
    assert_eq!(parse_data_type("#t\r\n", &mut 0)?, DataType::Boolean(true));
    assert_eq!(parse_data_type("#f\r\n", &mut 0)?, DataType::Boolean(false));

    Ok(())
}

#[test]
fn parse_double_type() -> Result<(), ParseError> {
    assert_eq!(parse_data_type(",10\r\n", &mut 0)?, DataType::Double(10.0));
    assert_eq!(parse_data_type(",1.11\r\n", &mut 0)?, DataType::Double(1.11));
    assert_eq!(parse_data_type(",inf\r\n", &mut 0)?, DataType::Double(f64::INFINITY));
    assert_eq!(parse_data_type(",-inf\r\n", &mut 0)?, DataType::Double(f64::NEG_INFINITY));

    let DataType::Double(nan) = parse_data_type(",nan\r\n", &mut 0)? else {
        return Err(ParseError::InvalidValue("Test case expected a double data type.".to_string()));
    };
    assert!(nan.is_nan());

    Ok(())
}

// TODO: More tests when more data types are added

//////////////////////////////////////////
// Marshall tests
//////////////////////////////////////////