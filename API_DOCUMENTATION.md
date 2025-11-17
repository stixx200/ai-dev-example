# Pet Management REST API Documentation

A fully functional REST API for managing pets built with Rust, Axum, and NX monorepo.

## 🚀 Quick Start

### Starting the Server

```bash
npx nx run my_app
```

The server will start on `http://localhost:3000`

### Running Tests

```bash
# Test all projects
npx nx run-many -t test

# Test specific projects
npx nx test data
npx nx test utils
npx nx test core
```

## 📚 API Endpoints

### Health Check

#### `GET /health`

Check if the API is running.

**Response:**

```json
{
  "status": "healthy",
  "service": "Pet Management API",
  "version": "1.0.0"
}
```

---

### Create Pet

#### `POST /pets`

Create a new pet in the system.

**Request Body:**

```json
{
  "name": "Buddy",
  "species": "Dog",
  "breed": "Golden Retriever", // Optional
  "age": 3, // Optional
  "owner_name": "John Doe",
  "owner_email": "john@example.com"
}
```

**Response (201 Created):**

```json
{
  "id": "decc649e-402c-49c2-a019-840e9669aaa0",
  "name": "Buddy",
  "species": "Dog",
  "breed": "Golden Retriever",
  "age": 3,
  "owner_name": "John Doe",
  "owner_email": "john@example.com",
  "created_at": "2025-11-17T14:22:22.187756Z",
  "updated_at": "2025-11-17T14:22:22.187756Z"
}
```

**Error Response (400 Bad Request):**

```json
{
  "error": "Pet name cannot be empty"
}
```

**Validation Rules:**

- `name`: Required, 1-100 characters after trimming
- `species`: Required, 1-50 characters after trimming
- `breed`: Optional, trimmed if provided
- `age`: Optional, must be ≤ 100 if provided
- `owner_name`: Required, 1-100 characters after trimming
- `owner_email`: Required, must contain '@' and '.'

---

### List All Pets

#### `GET /pets`

Retrieve a list of all pets.

**Response (200 OK):**

```json
{
  "count": 2,
  "pets": [
    {
      "id": "decc649e-402c-49c2-a019-840e9669aaa0",
      "name": "Buddy",
      "species": "Dog",
      "breed": "Golden Retriever",
      "age": 3,
      "owner_name": "John Doe",
      "owner_email": "john@example.com",
      "created_at": "2025-11-17T14:22:22.187756Z",
      "updated_at": "2025-11-17T14:22:22.187756Z"
    },
    {
      "id": "e95ccbc8-3d12-4fe0-8277-d289a8e8927e",
      "name": "Whiskers",
      "species": "Cat",
      "breed": "Siamese",
      "age": 2,
      "owner_name": "Jane Smith",
      "owner_email": "jane@example.com",
      "created_at": "2025-11-17T14:22:31.057825Z",
      "updated_at": "2025-11-17T14:22:31.057825Z"
    }
  ]
}
```

---

### Get Pet by ID

#### `GET /pets/{id}`

Retrieve a specific pet by its UUID.

**Path Parameters:**

- `id` (UUID): The unique identifier of the pet

**Response (200 OK):**

```json
{
  "id": "decc649e-402c-49c2-a019-840e9669aaa0",
  "name": "Buddy",
  "species": "Dog",
  "breed": "Golden Retriever",
  "age": 3,
  "owner_name": "John Doe",
  "owner_email": "john@example.com",
  "created_at": "2025-11-17T14:22:22.187756Z",
  "updated_at": "2025-11-17T14:22:22.187756Z"
}
```

**Error Response (404 Not Found):**

```json
{
  "error": "Pet with ID decc649e-402c-49c2-a019-840e9669aaa0 not found"
}
```

---

### Update Pet

#### `PUT /pets/{id}`

Update an existing pet's information. All fields are optional; only provided fields will be updated.

**Path Parameters:**

- `id` (UUID): The unique identifier of the pet

**Request Body:**

```json
{
  "name": "Mr. Whiskers", // Optional
  "species": "Cat", // Optional
  "breed": "Persian", // Optional
  "age": 3, // Optional
  "owner_name": "Jane Doe", // Optional
  "owner_email": "jane@new.com" // Optional
}
```

**Response (200 OK):**

```json
{
  "id": "e95ccbc8-3d12-4fe0-8277-d289a8e8927e",
  "name": "Mr. Whiskers",
  "species": "Cat",
  "breed": "Persian",
  "age": 3,
  "owner_name": "Jane Doe",
  "owner_email": "jane@new.com",
  "created_at": "2025-11-17T14:22:31.057825Z",
  "updated_at": "2025-11-17T14:23:17.234965Z"
}
```

**Error Responses:**

_404 Not Found:_

```json
{
  "error": "Pet with ID e95ccbc8-3d12-4fe0-8277-d289a8e8927e not found"
}
```

_400 Bad Request (validation error):_

```json
{
  "error": "Invalid email address: invalid-email"
}
```

---

### Delete Pet

#### `DELETE /pets/{id}`

Delete a pet from the system.

**Path Parameters:**

- `id` (UUID): The unique identifier of the pet

**Response (200 OK):**

```json
{
  "id": "e95ccbc8-3d12-4fe0-8277-d289a8e8927e",
  "name": "Mr. Whiskers",
  "species": "Cat",
  "breed": "Siamese",
  "age": 3,
  "owner_name": "Jane Smith",
  "owner_email": "jane@example.com",
  "created_at": "2025-11-17T14:22:31.057825Z",
  "updated_at": "2025-11-17T14:23:17.234965Z"
}
```

**Error Response (404 Not Found):**

```json
{
  "error": "Pet with ID e95ccbc8-3d12-4fe0-8277-d289a8e8927e not found"
}
```

---

## 🧪 Testing with cURL

### Create a Pet

```bash
curl -X POST http://localhost:3000/pets \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Buddy",
    "species": "Dog",
    "breed": "Golden Retriever",
    "age": 3,
    "owner_name": "John Doe",
    "owner_email": "john@example.com"
  }'
```

### List All Pets

```bash
curl http://localhost:3000/pets
```

### Get a Specific Pet

```bash
curl http://localhost:3000/pets/{pet-id}
```

### Update a Pet

```bash
curl -X PUT http://localhost:3000/pets/{pet-id} \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Updated Name",
    "age": 4
  }'
```

### Delete a Pet

```bash
curl -X DELETE http://localhost:3000/pets/{pet-id}
```

---

## 🏗️ Architecture

### Project Structure

```
rust-monorepo/
├── apps/
│   └── my_app/          # Main REST API server application
├── libs/
│   ├── api/             # REST API endpoints and routing (Axum)
│   ├── core/            # Business logic (PetStore with CRUD operations)
│   ├── data/            # Data models (Pet, CreatePetRequest, UpdatePetRequest)
│   └── utils/           # Validation and utility functions
```

### Technology Stack

- **Web Framework**: [Axum](https://github.com/tokio-rs/axum) 0.8
- **Async Runtime**: [Tokio](https://tokio.rs/)
- **Serialization**: [Serde](https://serde.rs/) with JSON support
- **HTTP Middleware**: Tower & Tower-HTTP (CORS, Tracing)
- **Logging**: Tracing & Tracing-Subscriber
- **Data Storage**: In-memory HashMap with RwLock (thread-safe)
- **Build System**: NX with Monodon Rust plugin

### Data Flow

1. **HTTP Request** → Axum Router (`libs/api`)
2. **Extractors** → Parse path params, JSON body
3. **Business Logic** → PetStore methods (`libs/core`)
4. **Validation** → Utility functions (`libs/utils`)
5. **Data Access** → In-memory HashMap (`libs/data`)
6. **HTTP Response** → JSON serialization via Serde

---

## 🔒 Features

### Validation

- Email format validation
- Name and species length constraints
- Age range validation
- Automatic whitespace trimming
- Comprehensive error messages

### Thread Safety

- Uses `parking_lot::RwLock` for thread-safe data access
- Multiple concurrent reads supported
- Write operations properly locked

### CORS Support

- Configured to allow all origins (development mode)
- Can be restricted for production use

### Logging

- Structured logging with tracing
- HTTP request/response logging
- Configurable via environment variables

---

## 🔧 Configuration

### Environment Variables

- `RUST_LOG`: Controls logging verbosity
  ```bash
  RUST_LOG=debug npx nx run my_app
  ```

### Server Port

Default: `0.0.0.0:3000`

To change, modify `apps/my_app/src/main.rs`:

```rust
let addr = "0.0.0.0:8080";  // Change port here
```

---

## 🧪 Running Tests

```bash
# Run all tests
npx nx run-many -t test

# Test with coverage
npx nx run-many -t test --coverage

# Test specific library
npx nx test core
npx nx test data
npx nx test utils
```

---

## 🚀 Production Considerations

### Current Implementation

- In-memory storage (data lost on restart)
- No authentication/authorization
- CORS allows all origins
- Single server instance

### Recommended Improvements for Production

1. **Database Integration**

   - Add PostgreSQL, MySQL, or MongoDB
   - Use SQLx or Diesel for database access
   - Implement proper migrations

2. **Authentication & Authorization**

   - Add JWT or session-based auth
   - Implement API keys
   - Add role-based access control

3. **CORS Configuration**

   - Restrict allowed origins
   - Configure allowed methods and headers

4. **Rate Limiting**

   - Add request rate limiting
   - Implement per-user quotas

5. **Monitoring & Observability**

   - Add metrics collection (Prometheus)
   - Implement distributed tracing
   - Set up error alerting

6. **Caching**

   - Add Redis for caching
   - Implement cache invalidation strategies

7. **API Versioning**

   - Version the API endpoints (`/v1/pets`)
   - Support multiple API versions

8. **Documentation**
   - Generate OpenAPI/Swagger documentation
   - Add API testing suite (Postman collection)

---

## 📦 Dependencies

### Main Dependencies

- `axum` - Web framework
- `tokio` - Async runtime
- `serde` - Serialization/deserialization
- `serde_json` - JSON support
- `tower` - Middleware
- `tower-http` - HTTP middleware (CORS, tracing)
- `tracing` - Structured logging
- `uuid` - UUID generation
- `chrono` - Date/time handling
- `parking_lot` - Thread synchronization

---

## 🐛 Troubleshooting

### Port Already in Use

If port 3000 is already in use:

```bash
# Find and kill the process
lsof -ti:3000 | xargs kill -9

# Or change the port in main.rs
```

### Build Errors

```bash
# Clean build artifacts
rm -rf dist/

# Rebuild
npx nx build my_app
```

### Test Failures

```bash
# Run tests with verbose output
npx nx test core --verbose
```

---

## 📄 License

MIT

---

## 👥 Contributing

This is a demonstration project for the NX Rust monorepo with Monodon plugin.

---

## 🎉 Summary

You now have a fully functional REST API with:

- ✅ Complete CRUD operations
- ✅ Input validation
- ✅ Error handling
- ✅ Thread-safe in-memory storage
- ✅ Structured logging
- ✅ CORS support
- ✅ Modular architecture with NX
- ✅ Comprehensive test suite

The API is production-ready for development/testing purposes and can be extended with database integration, authentication, and other production features as needed!
