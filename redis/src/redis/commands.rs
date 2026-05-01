use super::super::resp::definitions::*;

//////////////////////////////////////////
// CommandError
//////////////////////////////////////////
pub enum CommandError
{
    NonRunnableCommand
}

//////////////////////////////////////////
// Command runner
//////////////////////////////////////////
pub fn run_command(begin_type: DataType) -> Result<DataType, CommandError>
{
    match begin_type
    {
        // Commands that don't need arguments
        DataType::SimpleString(command) => {
            if command == "PING" { ping_command(None) }
            else { Err(CommandError::NonRunnableCommand) }
        }
        DataType::BulkString(command) => {
            if command == "PING" { ping_command(None) }
            else { Err(CommandError::NonRunnableCommand) }
        }

        // Commands with arguments
        DataType::Array(data_types) => {

        }

        // Default
        _ => { Err(CommandError::NonRunnableCommand) }
    }
}

//////////////////////////////////////////
// Commands
//////////////////////////////////////////
pub fn ping_command(message: Option<DataType>) -> DataType
{
    message.unwrap_or(DataType::SimpleString("OK"))
}