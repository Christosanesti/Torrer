// Tor control protocol parsing utilities

/// Parse Tor control protocol response
pub fn parse_response(response: &str) -> Result<Response, ParseError> {
    let lines: Vec<&str> = response.lines().collect();
    
    if lines.is_empty() {
        return Err(ParseError::EmptyResponse);
    }

    // Parse status code from first line
    let first_line = lines[0];
    let status_code = parse_status_code(first_line)?;
    
    // Parse data lines (if any)
    let data = if lines.len() > 1 {
        lines[1..].join("\n")
    } else {
        String::new()
    };

    Ok(Response {
        status_code,
        data,
    })
}

/// Parse status code from response line
fn parse_status_code(line: &str) -> Result<u16, ParseError> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return Err(ParseError::InvalidFormat);
    }

    parts[0].parse::<u16>()
        .map_err(|_| ParseError::InvalidFormat)
}

/// Tor control protocol response
#[derive(Debug, Clone)]
pub struct Response {
    pub status_code: u16,
    pub data: String,
}

/// Protocol parsing error
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Empty response")]
    EmptyResponse,
    #[error("Invalid response format")]
    InvalidFormat,
}

