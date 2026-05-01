mod resp;

fn main() {
    resp::parse_command("*2\r\n+AAAAAAA\r\n+BBBBBBBBBBB\r\n");
}

// For tests
#[cfg(test)]
mod tests;