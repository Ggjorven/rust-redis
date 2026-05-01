use super::definitions::*;

//////////////////////////////////////////
// ParseError
//////////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError
{
    InvalidSyntax(String),
    InvalidSize(String),
    NotImplemented,
}

//////////////////////////////////////////
// Parser
//////////////////////////////////////////
pub fn parse_command(command_str: &str) -> Result<Command, ParseError>
{
    let mut messages = command_str.split("\r\n");

    // First data type should be an array to begin the command
    let Some(array_str) = messages.next() else {
        return Err(ParseError::InvalidSyntax("Empty command passed in.".to_string()));
    };
    let DataType::Array(array_size) = parse_data_type(array_str)? else {
        return Err(ParseError::InvalidSyntax("Expected the command to start with an array size type (*<ARRAY SIZE>).".to_string()));
    };

    // Check array size
    if array_size == 0 {
        return Err(ParseError::InvalidSize("Invalid array size passed in (0).".to_string()));
    }

    // Retrieve the rest of the arguments
    let mut command_types: Vec<DataType> = Vec::with_capacity(array_size as usize);
    while let Some(message) = messages.next() {
        command_types.push(parse_data_type(message)?);
    }

    // Convert to DataValue's

    // Convert Command

    Err(ParseError::NotImplemented)
}

pub fn parse_data_type(message: &str) -> Result<DataType, ParseError>
{
    if message.len() == 0 { panic!("Internal logic error, received a message with length of 0."); }

    let mut chars = message.chars();
    match &chars.nth(0).unwrap() // First byte
    {
        '+' => {
            if message.len() == 1 { return Err(ParseError::InvalidSize("Cannot create a string with no content.".to_string())); }

            let string: &str = message.get(1..message.len()).unwrap();

            Ok(DataType::SimpleString(string.to_string()))
        },
        '-' => {
            if message.len() == 1 { return Err(ParseError::InvalidSize("Cannot create an error with no content.".to_string())); }

            let string: &str = message.get(1..message.len()).unwrap();

            Ok(DataType::SimpleError(string.to_string()))
        },
        ':' => {
            if message.len() == 1 { return Err(ParseError::InvalidSize("Integer doesn't have any content.".to_string())); }

            let value: i64 = message.get(1..message.len()).unwrap().parse::<i64>().map_err(|_parse_error| { return ParseError::InvalidSize("Cannot convert the passed in integer to an i64.".to_string()); })?;

            Ok(DataType::Integer(value))
        },
        '$' => {
            if message.len() == 1 { return Err(ParseError::InvalidSize("Creating a bulk string with no explicit size is not allowed.".to_string())); }

            let size: i64 = message.get(1..message.len()).unwrap().parse::<i64>().map_err(|_parse_error| { return ParseError::InvalidSize("Can't parse the size specified after the first byte.".to_string()); })?;

            // TODO: ...
            Ok(DataType::BulkString(size))
        },
        '*' => {
            if message.len() == 1 { return Err(ParseError::InvalidSize("Creating an array with no explicit size is not allowed.".to_string())); }

            let size: u64 = message.get(1..message.len()).unwrap().parse::<u64>().map_err(|_parse_error| { return ParseError::InvalidSize("Can't parse the size specified after the first byte.".to_string()); })?;

            Ok(DataType::Array(size))
        },
        '_' => { Ok(DataType::Null) },
        _ => { Err(ParseError::NotImplemented) }
    }
}

pub fn parse_data_value(data_types: &[&DataType]) -> Result<DataValue, ParseError>
{
    Err(ParseError::NotImplemented)
}