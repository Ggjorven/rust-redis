use std::process::ExitCode;

mod resp;
mod redis;
mod server;

#[tokio::main]
async fn main() -> ExitCode {
    let mut server = server::Server::new().await;
    
    let Ok(_) = server.listen(Some(6379)).await else { return ExitCode::FAILURE; };
    let Ok(_) = server.run_loop().await else { return ExitCode::FAILURE; };

    ExitCode::SUCCESS
}

// For tests
#[cfg(test)]
mod tests;