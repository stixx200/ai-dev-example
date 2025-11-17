use data::{CreatePetRequest, Pet, UpdatePetRequest};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use utils::*;
use uuid::Uuid;

/// Thread-safe in-memory pet store
#[derive(Clone)]
pub struct PetStore {
    pets: Arc<RwLock<HashMap<Uuid, Pet>>>,
}

impl PetStore {
    /// Creates a new empty pet store
    pub fn new() -> Self {
        PetStore {
            pets: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Creates a new pet with validation
    pub fn create_pet(&self, req: CreatePetRequest) -> Result<Pet, String> {
        // Validate all fields
        validate_pet_name(&req.name)?;
        validate_species(&req.species)?;
        validate_owner_name(&req.owner_name)?;
        
        if !validate_email(&req.owner_email) {
            return Err(format!("Invalid email address: {}", req.owner_email));
        }
        
        if let Some(age) = req.age {
            validate_age(age)?;
        }

        let pet = Pet::new(
            format_display_name(&req.name),
            format_display_name(&req.species),
            req.breed.map(|b| format_display_name(&b)),
            req.age,
            format_display_name(&req.owner_name),
            req.owner_email.trim().to_string(),
        );

        let mut pets = self.pets.write();
        pets.insert(pet.id, pet.clone());
        Ok(pet)
    }

    /// Retrieves a pet by ID
    pub fn get_pet(&self, id: &Uuid) -> Option<Pet> {
        let pets = self.pets.read();
        pets.get(id).cloned()
    }

    /// Lists all pets
    pub fn list_pets(&self) -> Vec<Pet> {
        let pets = self.pets.read();
        pets.values().cloned().collect()
    }

    /// Updates an existing pet
    pub fn update_pet(&self, id: &Uuid, req: UpdatePetRequest) -> Result<Pet, String> {
        // Validate all provided fields
        if let Some(ref name) = req.name {
            validate_pet_name(name)?;
        }
        if let Some(ref species) = req.species {
            validate_species(species)?;
        }
        if let Some(ref owner_name) = req.owner_name {
            validate_owner_name(owner_name)?;
        }
        if let Some(ref email) = req.owner_email {
            if !validate_email(email) {
                return Err(format!("Invalid email address: {}", email));
            }
        }
        if let Some(age) = req.age {
            validate_age(age)?;
        }

        let mut pets = self.pets.write();
        let pet = pets
            .get_mut(id)
            .ok_or_else(|| format!("Pet with ID {} not found", id))?;

        pet.update(
            req.name.map(|n| format_display_name(&n)),
            req.species.map(|s| format_display_name(&s)),
            req.breed.map(Some).map(|b| b.map(|v| format_display_name(&v))),
            req.age.map(Some),
            req.owner_name.map(|n| format_display_name(&n)),
            req.owner_email.map(|e| e.trim().to_string()),
        );

        Ok(pet.clone())
    }

    /// Deletes a pet by ID
    pub fn delete_pet(&self, id: &Uuid) -> Result<Pet, String> {
        let mut pets = self.pets.write();
        pets.remove(id)
            .ok_or_else(|| format!("Pet with ID {} not found", id))
    }

    /// Counts total pets
    pub fn count_pets(&self) -> usize {
        let pets = self.pets.read();
        pets.len()
    }
}

impl Default for PetStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_pet() {
        let store = PetStore::new();
        let req = CreatePetRequest {
            name: "Buddy".to_string(),
            species: "Dog".to_string(),
            breed: Some("Golden Retriever".to_string()),
            age: Some(3),
            owner_name: "John Doe".to_string(),
            owner_email: "john@example.com".to_string(),
        };

        let pet = store.create_pet(req).expect("Failed to create pet");
        assert_eq!(pet.name, "Buddy");
        assert_eq!(store.count_pets(), 1);
    }

    #[test]
    fn test_create_pet_invalid_email() {
        let store = PetStore::new();
        let req = CreatePetRequest {
            name: "Buddy".to_string(),
            species: "Dog".to_string(),
            breed: None,
            age: None,
            owner_name: "John Doe".to_string(),
            owner_email: "invalid-email".to_string(),
        };

        assert!(store.create_pet(req).is_err());
    }

    #[test]
    fn test_get_pet() {
        let store = PetStore::new();
        let req = CreatePetRequest {
            name: "Max".to_string(),
            species: "Cat".to_string(),
            breed: None,
            age: Some(2),
            owner_name: "Alice".to_string(),
            owner_email: "alice@example.com".to_string(),
        };

        let created_pet = store.create_pet(req).unwrap();
        let retrieved_pet = store.get_pet(&created_pet.id).unwrap();
        assert_eq!(created_pet.id, retrieved_pet.id);
        assert_eq!(retrieved_pet.name, "Max");
    }

    #[test]
    fn test_list_pets() {
        let store = PetStore::new();
        let req1 = CreatePetRequest {
            name: "Pet1".to_string(),
            species: "Dog".to_string(),
            breed: None,
            age: None,
            owner_name: "Owner1".to_string(),
            owner_email: "owner1@example.com".to_string(),
        };
        let req2 = CreatePetRequest {
            name: "Pet2".to_string(),
            species: "Cat".to_string(),
            breed: None,
            age: None,
            owner_name: "Owner2".to_string(),
            owner_email: "owner2@example.com".to_string(),
        };

        store.create_pet(req1).unwrap();
        store.create_pet(req2).unwrap();

        let pets = store.list_pets();
        assert_eq!(pets.len(), 2);
    }

    #[test]
    fn test_update_pet() {
        let store = PetStore::new();
        let req = CreatePetRequest {
            name: "Buddy".to_string(),
            species: "Dog".to_string(),
            breed: None,
            age: Some(3),
            owner_name: "John".to_string(),
            owner_email: "john@example.com".to_string(),
        };

        let pet = store.create_pet(req).unwrap();
        let update_req = UpdatePetRequest {
            name: Some("Max".to_string()),
            species: None,
            breed: Some("Labrador".to_string()),
            age: Some(4),
            owner_name: None,
            owner_email: None,
        };

        let updated_pet = store.update_pet(&pet.id, update_req).unwrap();
        assert_eq!(updated_pet.name, "Max");
        assert_eq!(updated_pet.age, Some(4));
        assert_eq!(updated_pet.breed, Some("Labrador".to_string()));
    }

    #[test]
    fn test_delete_pet() {
        let store = PetStore::new();
        let req = CreatePetRequest {
            name: "Buddy".to_string(),
            species: "Dog".to_string(),
            breed: None,
            age: None,
            owner_name: "John".to_string(),
            owner_email: "john@example.com".to_string(),
        };

        let pet = store.create_pet(req).unwrap();
        assert_eq!(store.count_pets(), 1);

        let deleted_pet = store.delete_pet(&pet.id).unwrap();
        assert_eq!(deleted_pet.id, pet.id);
        assert_eq!(store.count_pets(), 0);

        // Try to delete again
        assert!(store.delete_pet(&pet.id).is_err());
    }
}
