use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Represents a pet in the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pet {
    pub id: Uuid,
    pub name: String,
    pub species: String,
    pub breed: Option<String>,
    pub age: Option<u8>,
    pub owner_name: String,
    pub owner_email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Pet {
    /// Creates a new Pet instance with generated ID and timestamps
    pub fn new(
        name: String,
        species: String,
        breed: Option<String>,
        age: Option<u8>,
        owner_name: String,
        owner_email: String,
    ) -> Self {
        let now = Utc::now();
        Pet {
            id: Uuid::new_v4(),
            name,
            species,
            breed,
            age,
            owner_name,
            owner_email,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Updates the pet's information
    pub fn update(
        &mut self,
        name: Option<String>,
        species: Option<String>,
        breed: Option<Option<String>>,
        age: Option<Option<u8>>,
        owner_name: Option<String>,
        owner_email: Option<String>,
    ) {
        if let Some(n) = name {
            self.name = n;
        }
        if let Some(s) = species {
            self.species = s;
        }
        if let Some(b) = breed {
            self.breed = b;
        }
        if let Some(a) = age {
            self.age = a;
        }
        if let Some(on) = owner_name {
            self.owner_name = on;
        }
        if let Some(oe) = owner_email {
            self.owner_email = oe;
        }
        self.updated_at = Utc::now();
    }
}

/// Request payload for creating a new pet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePetRequest {
    pub name: String,
    pub species: String,
    pub breed: Option<String>,
    pub age: Option<u8>,
    pub owner_name: String,
    pub owner_email: String,
}

/// Request payload for updating an existing pet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePetRequest {
    pub name: Option<String>,
    pub species: Option<String>,
    pub breed: Option<String>,
    pub age: Option<u8>,
    pub owner_name: Option<String>,
    pub owner_email: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pet_creation() {
        let pet = Pet::new(
            "Buddy".to_string(),
            "Dog".to_string(),
            Some("Golden Retriever".to_string()),
            Some(3),
            "John Doe".to_string(),
            "john@example.com".to_string(),
        );
        assert_eq!(pet.name, "Buddy");
        assert_eq!(pet.species, "Dog");
        assert_eq!(pet.age, Some(3));
    }
    
    #[test]
    fn test_pet_update() {
        let mut pet = Pet::new(
            "Buddy".to_string(),
            "Dog".to_string(),
            Some("Golden Retriever".to_string()),
            Some(3),
            "John Doe".to_string(),
            "john@example.com".to_string(),
        );
        
        let original_updated_at = pet.updated_at;
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        pet.update(
            Some("Max".to_string()),
            None,
            None,
            Some(Some(4)),
            None,
            None,
        );
        
        assert_eq!(pet.name, "Max");
        assert_eq!(pet.age, Some(4));
        assert!(pet.updated_at > original_updated_at);
    }
}
