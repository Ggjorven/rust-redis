use super::super::resp;

#[test]
fn resp_parse_types() -> Result<(), resp::ParseError> {
    assert_eq!(resp::parse_data_type("*2")?, resp::DataType::Array(2));
    assert_eq!(resp::parse_data_type("*266")?, resp::DataType::Array(266));

    assert_eq!(resp::parse_data_type("_")?, resp::DataType::Null);

    Ok(())
}