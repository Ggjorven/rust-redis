mod resp;
mod redis;
mod server;

#[tokio::main]
async fn main() {
    let mut server = server::Server::new().await;
    server.listen(Some(6379)).await;
    server.run_loop().await;
}

// For tests
#[cfg(test)]
mod tests;