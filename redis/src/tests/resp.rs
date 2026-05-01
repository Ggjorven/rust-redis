use super::super::resp;

#[test]
fn resp_parse_simple_string_type() -> Result<(), resp::ParseError> {
    assert_eq!(resp::parse_data_type("+test\r\n", &mut 0)?, resp::DataType::SimpleString("test".to_string()));
    assert_eq!(resp::parse_data_type("+test message hihi\r\n", &mut 0)?, resp::DataType::SimpleString("test message hihi".to_string()));

    Ok(())
}

#[test]
fn resp_parse_simple_error_type() -> Result<(), resp::ParseError> {
    assert_eq!(resp::parse_data_type("-test\r\n", &mut 0)?, resp::DataType::SimpleError("test".to_string()));
    assert_eq!(resp::parse_data_type("-test message hihi\r\n", &mut 0)?, resp::DataType::SimpleError("test message hihi".to_string()));

    Ok(())
}

#[test]
fn resp_parse_integer_type() -> Result<(), resp::ParseError> {
    assert_eq!(resp::parse_data_type(":-1\r\n", &mut 0)?, resp::DataType::Integer(-1));
    assert_eq!(resp::parse_data_type(":0\r\n", &mut 0)?, resp::DataType::Integer(0));
    assert_eq!(resp::parse_data_type(":100\r\n", &mut 0)?, resp::DataType::Integer(100));

    Ok(())
}

#[test]
fn resp_parse_bulk_string_type() -> Result<(), resp::ParseError> {
    assert_eq!(resp::parse_data_type("$-1\r\n", &mut 0)?, resp::DataType::BulkString(None));
    assert_eq!(resp::parse_data_type("$5\r\nhello\r\n", &mut 0)?, resp::DataType::BulkString(Some("hello".to_string())));
    assert_eq!(resp::parse_data_type("$10\r\nhello\r\nyou\r\n", &mut 0)?, resp::DataType::BulkString(Some("hello\r\nyou".to_string())));

    Ok(())
}

#[test]
fn resp_parse_array_type() -> Result<(), resp::ParseError> {
    // assert_eq!(resp::parse_data_type("*2")?, resp::DataType::Array(2));
    // assert_eq!(resp::parse_data_type("*266")?, resp::DataType::Array(266));

    Ok(())
}

#[test]
fn resp_parse_null_type() -> Result<(), resp::ParseError> {
    // assert_eq!(resp::parse_data_type("_")?, resp::DataType::Null);

    Ok(())
}

