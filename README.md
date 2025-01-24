# Quickstart Rust Actix Web Project

This project demonstrates a basic CRUD REST API built using Rust, Actix Web, and PostgreSQL with Testcontainers for integration testing. It focuses on clean architecture principles, separating domain logic from infrastructure concerns.

## Project Structure

This project follows a modular structure to maintain clear separation of concerns. The main components are:

* **`register_user`:** This is the main crate, responsible for assembling dependencies and exposing the REST API.
* **`register_user/domain`:** Contains the core domain logic, including use cases and entities.
* **`register_user/primary/rest_server`:** Implements the REST API controllers and routes using Actix Web. It depends on the `domain` crate for business logic.
* **`register_user/secondary/postgresql`:** Provides the PostgreSQL database adapter for persistence, implementing the `domain`'s data access port.

## Getting Started

### Prerequisites

* Rust and Cargo installed (see [Rust Installation](https://www.rust-lang.org/tools/install)).
* Docker (for running Testcontainers).


### Running the Application

1.  Clone the repository: `git clone <repository_url>`
2.  Navigate to the project root directory: `cd quickstart-rust-actix-web`
3.  Navigate to the `register_user` directory: `cd register_user`
4.  Build and run: `cargo run`

The application will start and listen on the default port (8080).

### Running Tests

1.  Navigate to the project root directory: `cd quickstart-rust-actix-web`
2.  Navigate to the `register_user` directory: `cd register_user`

#### Unit Tests

Run unit tests within `src` directories (excluding doc tests and integration tests):
 - cargo test --lib -- --test-threads=1

#### Integration Tests

Integration tests reside in the `tests` directory and use Testcontainers. They are located at `register_user/tests`:
 - cargo test --tests -- --nocapture --test-threads=1

To run all tests across all crates and modules (including unit, integration, and doc tests if present):
 - cargo test --all

## API Endpoints
<details>
<summary>POST /users: Creates a new user.</summary>

<p><b>Request:</b></p>
<pre><code class="language-json">
{
  "name": "Test Name",
  "email": "test@test.com"
}
</code></pre>

<p><b>Response:</b></p>
<pre><code class="language-json">
{
  "id": "some-generated-uuid",
  "name": "Test Name",
  "email": "test@test.com"
}
</code></pre>
</details>

<details>
<summary>GET /users/{id}: Creates a new user.</summary>

<p><b>Response:</b></p>
<pre><code class="language-json">
{
  "count": 1,
  "users": [
    {
      "id": "some-generated-uuid",
      "name": "Test Name",
      "email": "test@test.com"
    }
  ]
}
</code></pre>
</details>

<details>
<summary>GET /users: Creates a new user.</summary>

<p><b>Response:</b></p>
<pre><code class="language-json">
{
  "count": 2,
  "users": [
    {
      "id": "some-generated-uuid",
      "name": "Test Name",
      "email": "test@test.com"
    },
    {
      "id": "some-generated-uuid",
      "name": "Test Name",
      "email": "test@test.com"
    }
  ]
}
</code></pre>
</details>

## Example Usage (with `curl`)

### Create a User
 - curl -X POST -H "Content-Type: application/json" -d '{"name": "John Doe", "email": "john.doe@example.com" } '  http://localhost:8080/ users

### Get a User by ID
curl http://localhost:8080/ users/ < user_ id> 

### Get All Users
 - curl http://localhost:8080/ users

## Architecture

### Basic Structure
```mermaid
graph LR A[REST Controller] --> B(Use Case); B --> C[Domain Entities]; B --> D(Data Access Port); D --> E[PostgreSQL Adapter];
```

### Ports and Adapters
```mermaid
graph LR subgraph Primary (Driving Adapters) A[REST Controller] --> B(Create User Use Case) A --> C(Get User Use Case) end
subgraph Domain (Business Logic) B --> D{User} C --> D end
subgraph Secondary (Driven Adapters) D --> E[PostgreSQL Adapter] end
```

## Dependencies


Key dependencies include:

*   `actix-web`: Web framework.
*   `serde`: Serialization/Deserialization.
*   `uuid`: UUID generation.
*   `testcontainers`: For integration testing with containers.
*   `tokio`: Asynchronous runtime.
*   `postgres`: PostgreSQL driver.



## Contributing

Contributions are welcome! Fork the repository, make changes, and submit pull requests.

## License

This project is licensed under the MIT License.

