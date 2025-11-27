use crate::error::{TorrerError, TorrerResult};
use crate::tor::TorClient;

/// Country-specific exit node selection
pub struct CountrySelector {
    country_code: Option<String>,
}

impl CountrySelector {
    /// Create a new CountrySelector
    pub fn new(country_code: Option<String>) -> Self {
        Self { country_code }
    }

    /// Validate country code (ISO 3166-1 alpha-2 format)
    pub fn validate_country_code(country_code: &str) -> TorrerResult<()> {
        // Check length
        if country_code.len() != 2 {
            return Err(TorrerError::Tor(
                format!("Invalid country code: '{}'. Must be exactly 2 letters (ISO 3166-1 alpha-2 format, e.g., CA, US, DE)", country_code)
            ));
        }

        // Check if all characters are alphabetic
        if !country_code.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(TorrerError::Tor(
                format!("Invalid country code: '{}'. Must contain only letters", country_code)
            ));
        }

        Ok(())
    }

    /// Validate multiple country codes (comma-separated)
    pub fn validate_country_codes(country_codes: &str) -> TorrerResult<Vec<String>> {
        let codes: Vec<&str> = country_codes.split(',').map(|s| s.trim()).collect();
        let mut validated = Vec::new();

        for code in codes {
            if code.is_empty() {
                continue;
            }
            Self::validate_country_code(code)?;
            validated.push(code.to_uppercase());
        }

        if validated.is_empty() {
            return Err(TorrerError::Tor("No valid country codes provided".to_string()));
        }

        Ok(validated)
    }

    /// Set exit node country (single or multiple)
    pub async fn set_exit_country(&self, client: &mut TorClient, country_code: &str) -> TorrerResult<()> {
        log::info!("Setting exit node country to: {}", country_code);

        // Validate country code(s)
        let validated_codes = if country_code.contains(',') {
            // Multiple countries
            Self::validate_country_codes(country_code)?
        } else {
            // Single country
            Self::validate_country_code(country_code)?;
            vec![country_code.to_uppercase()]
        };

        // Format country codes for Tor (comma-separated, wrapped in braces)
        let exit_nodes = if validated_codes.len() == 1 {
            format!("{{{}}}", validated_codes[0])
        } else {
            format!("{{{}}}", validated_codes.join(","))
        };
        
        // Set exit node country via Tor control port
        let command = format!("SETCONF ExitNodes={}\r\n", exit_nodes);
        let response = client.send_command(&command).await?;

        // Check for errors in response
        if response.contains("552") || response.contains("error") {
            return Err(TorrerError::Tor(
                format!("Failed to set exit country: {}", response)
            ));
        }

        log::info!("Exit node country set to: {}", exit_nodes);
        Ok(())
    }

    /// Clear exit node country restriction
    pub async fn clear_exit_country(&self, client: &mut TorClient) -> TorrerResult<()> {
        log::info!("Clearing exit node country restriction");
        
        let command = "SETCONF ExitNodes=\r\n";
        client.send_command(command).await?;

        log::info!("Exit node country restriction cleared");
        Ok(())
    }

    /// Get current exit country from Tor
    pub async fn get_exit_country(client: &mut TorClient) -> TorrerResult<Option<String>> {
        let command = "GETCONF ExitNodes\r\n";
        let response = client.send_command(command).await?;

        // Parse ExitNodes from response
        // Format: "250 ExitNodes={US}\r\n" or "250 ExitNodes=\r\n"
        for line in response.lines() {
            if line.starts_with("250 ExitNodes=") {
                let nodes = line.strip_prefix("250 ExitNodes=")
                    .and_then(|s| s.strip_suffix("\r\n"))
                    .unwrap_or("");
                
                if nodes.is_empty() || nodes == "{}" {
                    return Ok(None);
                }

                // Extract country codes from {US} or {US,CA} format
                let codes = nodes.trim_matches('{').trim_matches('}');
                if codes.is_empty() {
                    return Ok(None);
                }

                return Ok(Some(codes.to_string()));
            }
        }

        Ok(None)
    }
}

