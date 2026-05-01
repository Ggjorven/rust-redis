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
    // Get data types
    let mut types: Vec<DataType> = Vec::new();
    
    let mut i: usize = 0;
    while i < command_str.len()
    {
        types.push(parse_data_type(command_str, &mut i)?);
    }

    // TODO: Checks

    // TODO: Command

    Err(ParseError::NotImplemented)
}

pub fn parse_data_type(message: &str, index: &mut usize) -> Result<DataType, ParseError>
{
    match message.chars().nth(*index).unwrap()
    {
        '+' => {
            *index += 1;

            let Some((left, _right)) = message.get(*index..message.len()).unwrap_or("").split_once("\r\n") else {
                return Err(ParseError::InvalidSyntax("Expected a simple string ended with \\r\\n, but found None.".to_string()))
            };

            *index += left.len() + 2;

            Ok(DataType::SimpleString(left.to_string()))
        },
        '-' => {
            *index += 1;

            let Some((left, _right)) = message.get(*index..message.len()).unwrap_or("").split_once("\r\n") else {
                return Err(ParseError::InvalidSyntax("Expected a simple error ended with \\r\\n, but found None.".to_string()))
            };

            *index += left.len() + 2;

            Ok(DataType::SimpleError(left.to_string()))
        },
        ':' => {
            *index += 1;

            let Some((left, _right)) = message.get(*index..message.len()).unwrap_or("").split_once("\r\n") else {
                return Err(ParseError::InvalidSyntax("Expected an integer ended with \\r\\n, but found None.".to_string()))
            };

            *index += left.len() + 2;

            let value: i64 = left.parse::<i64>().map_err(|_parse_error| { return ParseError::InvalidSize("Cannot convert the passed in integer to an i64.".to_string()); })?;

            Ok(DataType::Integer(value))
        },
        '$' => {
            *index += 1;

            let Some((left, _right)) = message.get(*index..message.len()).unwrap_or("").split_once("\r\n") else {
                return Err(ParseError::InvalidSyntax("Expected a size for the bulk string ended with \\r\\n, but found None.".to_string()))
            };

            *index += left.len() + 2;

            let bulk_size: i64 = left.parse::<i64>().map_err(|_parse_error| { return ParseError::InvalidSize("Cannot convert the bulk size to an i64.".to_string()); })?;
            
            if bulk_size < 0
            {
                if bulk_size == -1 { *index += 2; return Ok(DataType::BulkString(None)); }
                else { return Err(ParseError::InvalidSize("Cannot have a negative number as a bulk string size.".to_string())); }
            }

            let Some(string) = message.get(*index..*index + bulk_size as usize) else {
                return Err(ParseError::InvalidSize("Size specified in bulk size is too large compared to the passed in string.".to_string()));
            };

            *index += bulk_size as usize + 2;

            Ok(DataType::BulkString(Some(string.to_string())))
        },
        '*' => {
            *index += 1;

            let Some((left, _right)) = message.get(*index..message.len()).unwrap_or("").split_once("\r\n") else {
                return Err(ParseError::InvalidSyntax("Expected a size for the array ended with \\r\\n, but found None.".to_string()))
            };

            *index += left.len() + 2;

            let array_size: u64 = left.parse::<u64>().map_err(|_parse_error| { return ParseError::InvalidSize("Cannot convert the array size to a u64.".to_string()); })?;
            
            let mut data_types: Vec<DataType> = Vec::with_capacity(array_size as usize);

            let mut i = 0;
            while i < array_size
            {
                data_types.push(parse_data_type(message, index)?);
                i += 1;
            }
            
            Ok(DataType::Array(data_types))
        },
        '_' => { *index += 1 + 2; Ok(DataType::Null) },
        _ => { Err(ParseError::NotImplemented) }
    }
}