use super::definitions::*;

//////////////////////////////////////////
// ParseError
//////////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError
{
    InvalidSyntax(String),
    InvalidValue(String),
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
        // RESP2
        '+' => {
            *index += 1;

            let Some((left, _right)) = message.get(*index..message.len()).unwrap_or("").split_once("\r\n") else {
                return Err(ParseError::InvalidSyntax("Expected a simple string ended with \\r\\n, but found None.".to_string()));
            };

            *index += left.len() + 2;

            Ok(DataType::SimpleString(left.to_string()))
        },
        '-' => {
            *index += 1;

            let Some((left, _right)) = message.get(*index..message.len()).unwrap_or("").split_once("\r\n") else {
                return Err(ParseError::InvalidSyntax("Expected a simple error ended with \\r\\n, but found None.".to_string()));
            };

            *index += left.len() + 2;

            Ok(DataType::SimpleError(left.to_string()))
        },
        ':' => {
            *index += 1;

            let Some((left, _right)) = message.get(*index..message.len()).unwrap_or("").split_once("\r\n") else {
                return Err(ParseError::InvalidSyntax("Expected an integer ended with \\r\\n, but found None.".to_string()));
            };

            *index += left.len() + 2;

            let value: i64 = left.parse::<i64>().map_err(|_parse_error| { return ParseError::InvalidValue("Cannot convert the passed in integer to an i64.".to_string()); })?;

            Ok(DataType::Integer(value))
        },
        '$' => {
            *index += 1;

            let Some((left, _right)) = message.get(*index..message.len()).unwrap_or("").split_once("\r\n") else {
                return Err(ParseError::InvalidSyntax("Expected a size for the bulk string ended with \\r\\n, but found None.".to_string()))
            };

            *index += left.len() + 2;

            let bulk_size: i64 = left.parse::<i64>().map_err(|_parse_error| { return ParseError::InvalidValue("Cannot convert the bulk size to an i64.".to_string()); })?;
            
            if bulk_size < 0
            {
                if bulk_size == -1 { *index += 2; return Ok(DataType::BulkString(None)); }
                else { return Err(ParseError::InvalidValue("Cannot have a negative number as a bulk string size.".to_string())); }
            }

            let Some(string) = message.get(*index..*index + bulk_size as usize) else {
                return Err(ParseError::InvalidValue("Size specified in bulk size is too large compared to the passed in string.".to_string()));
            };

            *index += bulk_size as usize + 2;

            Ok(DataType::BulkString(Some(string.to_string())))
        },
        '*' => {
            *index += 1;

            let Some((left, _right)) = message.get(*index..message.len()).unwrap_or("").split_once("\r\n") else {
                return Err(ParseError::InvalidSyntax("Expected a size for the array ended with \\r\\n, but found None.".to_string()));
            };

            *index += left.len() + 2;

            // NOTE: https://redis.io/docs/latest/develop/reference/protocol-spec/#null-arrays
            // Maybe change to i64 to support RESP2 null array and change the DataType accordingly to an optional
            let array_size: u64 = left.parse::<u64>().map_err(|_parse_error| { return ParseError::InvalidValue("Cannot convert the array size to a u64.".to_string()); })?;
            
            let mut data_types: Vec<DataType> = Vec::with_capacity(array_size as usize);

            let mut i = 0;
            while i < array_size
            {
                data_types.push(parse_data_type(message, index)?);
                i += 1;
            }
            
            Ok(DataType::Array(data_types))
        },

        // RESP3
        '_' => { *index += 1 + 2; Ok(DataType::Null) },
        '#' => {
            *index += 1;

            let Some(t_or_f) = message.chars().nth(*index) else {
                return Err(ParseError::InvalidSyntax("Expected a 't' or 'f' after the #.".to_string()));
            };

            match t_or_f
            {
                't' => { Ok(DataType::Boolean(true)) },
                'f' => { Ok(DataType::Boolean(false)) },
                _ => { Err(ParseError::InvalidValue("Expected a 't' or 'f' after the #.".to_string())) }
            }
        },
        ',' => {
            *index += 1;
            
            let Some((left, _right)) = message.get(*index..message.len()).unwrap_or("").split_once("\r\n") else {
                return Err(ParseError::InvalidSyntax("Expected a size for the array ended with \\r\\n, but found None.".to_string()));
            };
            
            *index += left.len() + 2;
            
            // NOTE: [<E|e>[sign]<exponent>] is not implemented?
            let value: f64 = left.parse::<f64>().map_err(|_parse_error| { return ParseError::InvalidValue("Cannot convert the input to a f64.".to_string()); })?;
        
            Ok(DataType::Double(value))
        },

        _ => { Err(ParseError::NotImplemented) }
    }
}