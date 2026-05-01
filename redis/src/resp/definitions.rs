//////////////////////////////////////////
// RESP DataType
//////////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub enum DataType
{
    // First byte specified by comment

    // RESP2
    SimpleString(String),       // +
    SimpleError(String),        // -
    Integer(i64),               // :
    BulkString(Option<String>), // $ // NOTE: Can be None (nil) when its -1
    Array(Vec<DataType>),       // *
 
    // RESP3
    Null,                       // _
    Boolean(bool),              // #
    Double(f64),                // ,

    // TODO: ...
}

//////////////////////////////////////////
// RESP DataValue
//////////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub enum DataValue
{
    // First byte specified by comment

    // RESP2
    SimpleString(String),       // +
    SimpleError(String),        // -
    Integer(i64),               // :
    BulkString(Vec<String>),    // $ // NOTE: Can be empty() for null bulk strings
    Array(Vec<DataValue>),       // * // NOTE: Can be mixed types and arrays of arrays
 
    // RESP3
    Null,                       // _
    Boolean(bool),              // #
    Double(f64),                // ,

    // TODO: ...
}

//////////////////////////////////////////
// RESP Commands
//////////////////////////////////////////
#[derive(Debug, Clone)]
pub struct GetCommand // GET
{
    key: String
}

#[derive(Debug, Clone)]
pub struct SetCommand // SET
{
    key: String,
    value: DataType
}

#[derive(Debug, Clone)]
pub struct DelCommand // DEL
{
    key: String
}

#[derive(Debug, Clone)]
pub enum Command 
{
    Get(GetCommand),
    Set(SetCommand),
    Del(DelCommand),
    
    Ping
}