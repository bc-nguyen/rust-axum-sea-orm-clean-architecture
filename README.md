# Rust REST API Skeleton

This project is a REST API skeleton written in Rust, designed with a clean, layered architecture. It serves as a robust starting point for building scalable and maintainable web services. It features [axum](https://docs.rs/axum/latest/axum/) for the web framework, [sea-orm](sea-orm) for database interaction, and a clear separation of concerns inspired by Clean Architecture principles.

## Features

- **Layered Architecture**: Separates Domain, Application, Infrastructure, and Presentation layers.
- **Web Framework**: Built with `axum`, a modern and ergonomic web framework.
- **Async Everywhere**: Fully asynchronous for high performance.
- **Database ORM**: Uses `sea-orm` for type-safe, asynchronous database access.
- **Configuration**: Simple configuration management via `.env` files.
- **Authentication**: JWT-based authentication middleware (`auth_guard`).
- **Validation**: Built-in request validation for query parameters and JSON bodies.
- **Dependency Injection**: Centralized application state (`AppState`) for managing dependencies.
- **Task Automation**: Uses `Taskfile.dev` for common development tasks like migrations and running the server.
- **Comprehensive Testing**: Includes unit and integration tests with `sea-orm`'s mock database.

## Project Structure

The project follows a layered architecture to promote separation of concerns, testability, and maintainability.

```
├── src
│   ├── application         # Business Logic Layer
│   │   ├── cases           # Use cases (e.g., AddCompany, QueryCompany)
│   │   ├── dtos            # Data Transfer Objects for use case inputs/outputs
│   │   └── error.rs        # Application-specific errors
│   │
│   ├── domain              # Core Business Layer
│   │   ├── organization    # Business sub-domain (e.g., Company, Department)
│   │   │   └── repositories  # Repository traits (interfaces)
│   │   └── error.rs        # Domain-specific errors
│   │
│   ├── infrastructure      # Implementation Layer
│   │   ├── db              # Database logic (SeaORM entities, repository implementations)
│   │   └── helpers         # Utility modules (e.g., JWT token helper)
│   │
│   └── presentation        # API Layer (HTTP)
│       ├── guards          # Authentication and other guards
│       ├── handlers        # HTTP handlers that orchestrate API requests
│       ├── http        # Http server and routes
│       ├── middlewares     # Request/response middleware (e.g., validation)
│       └── response.rs     # Standardized API response structures
│
├── tests                   # Unit and Integration Tests
│   ├── application
│   │   └── cases           # Tests for use cases
│   └── infrastructure
│       ├── implements      # Tests for repository implementations
│       └── helpers         # Tests for helper modules
│
├── migrations              # Database migration files
├── Taskfile.yml            # Task definitions for automation
└── Cargo.toml              # Rust project manifest
```

### Layer Explanations

1.  **`domain`**: Contains the business entities and the repository traits that define how the application interacts with data, without knowing the implementation details.

2.  **`application`**: This layer contains the specific business logic of the application, orchestrated through **Use Cases** (e.g., `AddCompanyUseCase`). Use cases execute a single business operation. They depend on the `domain` layer's repository traits to access data and use **DTOs** for input and output.

3.  **`infrastructure`**: This layer provides concrete implementations for the interfaces defined in the `domain` layer, and other utility modules.

4.  **`presentation`**: This is the outermost layer, responsible for handling external interactions, primarily HTTP requests in this case. It uses `axum` to define routes and handlers. Handlers are responsible for parsing requests, calling the appropriate application use case, and formatting the response.

## Getting Started

### Prerequisites

- **Rust**: Install the Rust toolchain. See [rustup](https://rustup.rs/).
- **Task**: Install [task](https://taskfile.dev/) for running development commands. See Taskfile.dev.
- **Podman** or **Docker**: Required to run the PostgreSQL database.
- **`sea-orm-cli`**: Install the SeaORM command-line tool.
  ```sh
  cargo install sea-orm-cli@^2.0.0-rc
  ```
- **`cargo-llvm-cov`**: Install the  cargo-llvm-cov  for coverage report.
  ```sh
  cargo install  cargo-llvm-cov
  ```

### Installation & Running

1.  **Set up environment variables:**
    Copy the example environment file and update it with your database credentials.
    ```sh
    cp .env.example .env.local
    ```
    Your `.env.local` should look like this:
    ```env
    DB_CONNECT_STR=postgres://user:password@localhost:5432/db_name
    ```

3.  **Start the database and the application:**
    This command uses `podman compose` (or `docker compose`) to start a PostgreSQL container and then uses `cargo watch` to run the application, automatically restarting it on file changes.
    ```sh
    task local:start
    ```

The API server will be running at `http://127.0.0.1:8000`.

## Development Workflow

### Running Tests

Execute all unit and integration tests:
```sh
task ut:run
```

To get a test coverage report in your terminal:
```sh
task ut:cov
```

To generate and open an HTML coverage report:
```sh
task ut:cov-html
```

### Database Migrations

When you need to make a change to the database schema, first generate a new migration file:
```sh
task migrate:add -- <MIGRATION_NAME>
```

Edit the generated file in the `migrations/` directory, then apply it:
```sh
task migrate:run
```

### Generating Entities

After running migrations, you can regenerate the `sea-orm` entity files from the live database schema:
```sh
task entity:generate
```

## How to Add a New Feature (Example: "Product")

1.  **Domain Layer**:
    -   Define `Product` entity properties if needed.
    -   In `src/domain/`, create a `product_repo.rs` trait with methods like `add`, `find_by_id`, etc.

2.  **Infrastructure Layer**:
    -   Run `task migrate:add -- CreateProductTable` and `task migrate:run`.
    -   Run `task entity:generate` to create the `products` entity for SeaORM.
    -   In `src/infrastructure/db/implements/`, create `product_repo_impl.rs` that implements the `ProductRepository` trait using SeaORM.

3.  **Application Layer**:
    -   In `src/application/dtos/`, create `product_dtos.rs` for `ReqAddProductDto`, etc.
    -   In `src/application/cases/`, create `add_product.rs` which defines the `AddProductUseCase`. This use case will call the `product_repo.add()` method.

4.  **Presentation Layer**:
    -   Define a suitable route handler`src/presentation/http/routes`

5.  **Testing**:
    -   Add tests for the `ProductRepository` implementation in `tests/infrastructure/implements/`.
    -   Add tests for the `AddProductUseCase` in `tests/application/cases/`.