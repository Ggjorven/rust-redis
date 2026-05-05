use tokio::net::*;

use super::redis::*;

//////////////////////////////////////////
// ServerError
//////////////////////////////////////////
pub enum ServerError
{
    FailedToBind(String),
    NotListening
}

//////////////////////////////////////////
// Server
//////////////////////////////////////////
#[derive(Debug)]
pub struct Server
{
    database: Database,
    listener: Option<TcpListener>
}

impl Server
{
    pub async fn new() -> Self
    {
        Self {
            database: Database::new(),
            listener: None
        }
    }

    pub async fn listen(&mut self, port: Option<u16>) -> Result<(), ServerError>
    {
        let port = port.unwrap_or(6379);
        let full_address = format!("127.0.0.1:{}", port);

        let listener = TcpListener::bind(full_address).await;
        match listener
        {
            Ok(listener) => { self.listener = Some(listener); Ok(()) },
            Err(error) => { Err(ServerError::FailedToBind(format!("{}", error.kind()))) }
        }
    }

    pub async fn run_loop(&mut self) -> Result<(), ServerError>
    {
        let Some(ref listener) = self.listener else { return Err(ServerError::NotListening); };

        loop {
            let connection_result = listener.accept().await;
            match connection_result
            {
                Ok((socket, _)) => 
                { 
                    let db = self.database.copy();

                    tokio::spawn(async move {
                        Server::handle_connection(socket, db).await;
                    });
                },
                Err(error) => { eprintln!("{}", error.kind()); }
            }
        }
    }

    async fn handle_connection(socket: TcpStream, database: Database)
    {
        
    }
}