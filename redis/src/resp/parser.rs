use std::str::Chars;

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
        // '*' => {
        //     if message.len() == 1 { return Err(ParseError::InvalidSize("Creating an array with no explicit size is not allowed.".to_string())); }
// 
        //     let size: u64 = message.get(1..message.len()).unwrap().parse::<u64>().map_err(|_parse_error| { return ParseError::InvalidSize("Can't parse the size specified after the first byte.".to_string()); })?;
// 
        //     Ok(DataType::Array(size))
        // },
        '_' => { Ok(DataType::Null) },
        _ => { Err(ParseError::NotImplemented) }
    }
}