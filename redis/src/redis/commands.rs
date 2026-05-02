use super::*;
use super::super::resp::*;

//////////////////////////////////////////
// CommandError
//////////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
pub enum CommandError
{
    InvalidArgumentCount(String),
    InvalidArgumentType(String),
    UnknownCommand(String),
    DatabaseError(DatabaseError),
    NonRunnableCommand
}

//////////////////////////////////////////
// Command runner
//////////////////////////////////////////
pub fn run_command(db: &mut Database, begin_type: DataType) -> Result<DataType, CommandError>
{
    match begin_type
    {
        // Commands that don't need arguments
        DataType::SimpleString(command) | DataType::BulkString(Some(command)) => { run_non_arg_command(db, command.as_str()) },

        // Commands with arguments
        DataType::Array(data_types) => {
            if data_types.len() == 0 { 
                Err(CommandError::NonRunnableCommand)
            }
            else {
                match data_types.get(0).unwrap()
                {
                    // Commands
                    DataType::SimpleString(command) | DataType::BulkString(Some(command)) => { 
                        if data_types.len() == 1 { 
                            run_non_arg_command(db, command.as_str())
                        }
                        else {
                            run_arg_command(db, command.as_str(), &data_types[1..data_types.len()])
                        }
                    },

                    // Default
                    _ => { Err(CommandError::NonRunnableCommand) }
                }
            }
        }

        // Default
        _ => { Err(CommandError::NonRunnableCommand) }
    }
}

fn run_non_arg_command(db: &mut Database, command: &str) -> Result<DataType, CommandError>
{
    match command
    {
        "PING" => { Ok(ping_command(None)) },
        _ => { Err(CommandError::UnknownCommand(format!("Command \"{}\" does not exist.", command))) }
    }
}

fn run_arg_command(db: &mut Database, command: &str, arguments: &[DataType]) -> Result<DataType, CommandError>
{
    match command
    {
        "PING" => 
        { 
            if arguments.len() != 1 { return Err(CommandError::InvalidArgumentCount(format!("PING command expects 1 argument but got {}.", arguments.len()))); }
            
            let arg0 = arguments.get(0).unwrap();
            if let DataType::SimpleString(argument) | DataType::BulkString(Some(argument)) = arg0 {
                Ok(ping_command(Some(argument.as_str())))
            }
            else {
                Err(CommandError::InvalidArgumentType(format!("PING expects a SimpleString or BulkString, got a {:?}.", arg0)))                
            }
        },
        "SET" => 
        { 
            if arguments.len() != 2 { return Err(CommandError::InvalidArgumentCount(format!("SET command expects 2 argument but got {}.", arguments.len()))); }
            
            let arg0 = arguments.get(0).unwrap();
            if let DataType::SimpleString(name) | DataType::BulkString(Some(name)) = arg0 {
                set_command(db, name, arguments.get(1).unwrap())
            }
            else {
                Err(CommandError::InvalidArgumentType(format!("SET expects a SimpleString or BulkString as the first argument, got a {:?}.", arg0)))                
            }
        },
        _ => { Err(CommandError::UnknownCommand(format!("Command \"{}\" does not exist.", command))) }
    }
}

//////////////////////////////////////////
// Commands
//////////////////////////////////////////
pub fn ping_command(message: Option<&str>) -> DataType
{
    DataType::SimpleString(message.unwrap_or("OK").to_string())
}

pub fn set_command(db: &mut Database, name: &str, value: &DataType) -> Result<DataType, CommandError>
{
    let result = db.set(name, value);

    if let Err(error) = result {
        Err(CommandError::DatabaseError(error))
    }
    else {
        Ok(DataType::SimpleString("OK".to_string()))
    }
}