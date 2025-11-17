/// Validates an email address format
pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

/// Formats a display name by trimming whitespace
pub fn format_display_name(name: &str) -> String {
    name.trim().to_string()
}

/// Validates a pet name (must not be empty after trimming)
pub fn validate_pet_name(name: &str) -> Result<(), String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("Pet name cannot be empty".to_string());
    }
    if trimmed.len() > 100 {
        return Err("Pet name must be 100 characters or less".to_string());
    }
    Ok(())
}

/// Validates a pet species
pub fn validate_species(species: &str) -> Result<(), String> {
    let trimmed = species.trim();
    if trimmed.is_empty() {
        return Err("Species cannot be empty".to_string());
    }
    if trimmed.len() > 50 {
        return Err("Species must be 50 characters or less".to_string());
    }
    Ok(())
}

/// Validates pet age
pub fn validate_age(age: u8) -> Result<(), String> {
    if age > 100 {
        return Err("Age must be 100 or less".to_string());
    }
    Ok(())
}

/// Validates owner name
pub fn validate_owner_name(name: &str) -> Result<(), String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err("Owner name cannot be empty".to_string());
    }
    if trimmed.len() > 100 {
        return Err("Owner name must be 100 characters or less".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email() {
        assert!(validate_email("test@example.com"));
        assert!(!validate_email("invalid-email"));
        assert!(!validate_email("no-at-sign.com"));
    }

    #[test]
    fn test_format_display_name() {
        assert_eq!(format_display_name("  John Doe  "), "John Doe");
        assert_eq!(format_display_name("Alice"), "Alice");
    }
    
    #[test]
    fn test_validate_pet_name() {
        assert!(validate_pet_name("Buddy").is_ok());
        assert!(validate_pet_name("  Max  ").is_ok());
        assert!(validate_pet_name("").is_err());
        assert!(validate_pet_name("   ").is_err());
        assert!(validate_pet_name(&"A".repeat(101)).is_err());
    }
    
    #[test]
    fn test_validate_species() {
        assert!(validate_species("Dog").is_ok());
        assert!(validate_species("Cat").is_ok());
        assert!(validate_species("").is_err());
        assert!(validate_species(&"A".repeat(51)).is_err());
    }
    
    #[test]
    fn test_validate_age() {
        assert!(validate_age(0).is_ok());
        assert!(validate_age(50).is_ok());
        assert!(validate_age(100).is_ok());
        assert!(validate_age(101).is_err());
    }
    
    #[test]
    fn test_validate_owner_name() {
        assert!(validate_owner_name("John Doe").is_ok());
        assert!(validate_owner_name("").is_err());
        assert!(validate_owner_name(&"A".repeat(101)).is_err());
    }
}
