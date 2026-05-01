use super::super::resp;

#[test]
fn resp_parse_types() -> Result<(), resp::ParseError> {
    assert_eq!(resp::parse_data_type("+test")?, resp::DataType::SimpleString("test".to_string()));
    assert_eq!(resp::parse_data_type("+test message hihi")?, resp::DataType::SimpleString("test message hihi".to_string()));

    assert_eq!(resp::parse_data_type("-test")?, resp::DataType::SimpleError("test".to_string()));
    assert_eq!(resp::parse_data_type("-test message hihi")?, resp::DataType::SimpleError("test message hihi".to_string()));

    assert_eq!(resp::parse_data_type(":-1")?, resp::DataType::Integer(-1));
    assert_eq!(resp::parse_data_type(":0")?, resp::DataType::Integer(0));
    assert_eq!(resp::parse_data_type(":100")?, resp::DataType::Integer(100));

    // TODO: Bulk string

    assert_eq!(resp::parse_data_type("*2")?, resp::DataType::Array(2));
    assert_eq!(resp::parse_data_type("*266")?, resp::DataType::Array(266));

    assert_eq!(resp::parse_data_type("_")?, resp::DataType::Null);

    Ok(())
}