# Rust Monorepo with NX and Monodon

This workspace demonstrates a modern Rust monorepo setup using NX and the Monodon plugin, with a complete CI/CD pipeline for GitHub Actions.

## 🏗️ Architecture

The workspace follows a modular architecture with clear separation of concerns:

```
rust-monorepo/
├── apps/
│   └── my_app/          # Main Rust binary application
├── libs/
│   ├── core/            # Core business logic (depends on utils & data)
│   ├── data/            # Data structures and models
│   └── utils/           # Utility functions
└── .github/
    └── workflows/
        └── ci.yml       # GitHub Actions CI pipeline
```

### Project Dependencies

```
my_app (application)
  ├── core (library)
  │   ├── data (library)
  │   └── utils (library)
  └── data (library)
```

## 🚀 Features

- **NX Monorepo**: Intelligent build system with caching and task orchestration
- **Monodon Plugin**: Rust support for NX with Cargo integration
- **Modular Libraries**: Functionality split into reusable libraries
- **CI/CD Pipeline**: GitHub Actions workflow with Rust toolchain support
- **Dependency Graph**: Visual representation of project dependencies
- **Caching**: NX caching for faster builds and tests

## 📦 Projects

### Applications

#### `my_app`

The main Rust binary application that demonstrates the use of all libraries. It creates users with validation and processes them using the core library.

**Key Features:**

- User creation with validation
- Processing and displaying user information
- Integration with all library components

### Libraries

#### `data`

Contains data structures and models used across the workspace.

**Exports:**

- `User` struct: Represents a user with id, name, and email

#### `utils`

Utility functions for common operations.

**Exports:**

- `validate_email()`: Email validation
- `format_display_name()`: Name formatting

#### `core`

Core business logic that orchestrates functionality from other libraries.

**Exports:**

- `create_user()`: Creates validated users
- `process_users()`: Batch processes user data

## 🛠️ Commands

### Building

```bash
# Build all projects
npx nx run-many -t build

# Build a specific project
npx nx build my_app
npx nx build core
```

### Testing

```bash
# Run all tests
npx nx run-many -t test

# Test a specific project
npx nx test my_app
npx nx test core
```

### Linting

```bash
# Lint all projects
npx nx run-many -t lint

# Lint a specific project
npx nx lint my_app
```

### Running

```bash
# Run the main application
npx nx run my_app
```

### Project Graph

```bash
# View the project dependency graph
npx nx graph

# Generate static HTML graph
npx nx graph --file=project-graph.html
```

## 🔄 CI/CD Pipeline

The workspace includes a GitHub Actions workflow (`.github/workflows/ci.yml`) that:

1. Sets up Rust toolchain (stable)
2. Caches Cargo dependencies for faster builds
3. Installs Node.js dependencies
4. Runs affected tasks only (lint, test, build)
5. Provides automatic fix suggestions via `nx fix-ci`

### CI Features

- **Rust Toolchain**: Automatically installs stable Rust via `dtolnay/rust-toolchain`
- **Cargo Caching**: Uses `Swatinem/rust-cache` for faster builds
- **Smart Builds**: Only builds/tests affected projects using NX
- **Self-Healing CI**: Suggests fixes for failures

## 📊 NX Features in Use

### Caching

NX automatically caches build outputs and test results. When you run a task that hasn't changed, NX retrieves results from cache instantly.

### Affected Commands

Run tasks only on projects affected by your changes:

```bash
npx nx affected -t test    # Test only affected projects
npx nx affected -t build   # Build only affected projects
```

### Task Dependencies

NX automatically understands and respects task dependencies. When you build `my_app`, it automatically builds `core`, `data`, and `utils` first.

## 🔧 Development Workflow

1. **Create a feature branch**

   ```bash
   git checkout -b feature/new-functionality
   ```

2. **Make changes to libraries or apps**

   ```bash
   # Edit files in libs/ or apps/
   ```

3. **Run affected tests**

   ```bash
   npx nx affected -t test
   ```

4. **Build affected projects**

   ```bash
   npx nx affected -t build
   ```

5. **Commit and push**

   ```bash
   git add .
   git commit -m "Add new functionality"
   git push origin feature/new-functionality
   ```

6. **CI runs automatically** on pull request

## 🎯 Adding New Projects

### Add a New Library

```bash
npx nx generate @monodon/rust:library my-new-lib
```

### Add a New Binary Application

```bash
npx nx generate @monodon/rust:binary my-new-app
```

### Add a New Library with NAPI (Node.js bindings)

```bash
npx nx generate @monodon/rust:library my-node-lib --napi
```

## 📚 Resources

- [NX Documentation](https://nx.dev)
- [Monodon Plugin](https://www.npmjs.com/package/@monodon/rust)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [GitHub Actions](https://docs.github.com/en/actions)

## 🧪 Example Output

When you run the application with `npx nx run my_app`, you'll see:

```
Welcome to My Rust Application!
================================

Processed 3 users:
  1. Alice Smith (alice@example.com)
  2. Bob Johnson (bob@example.com)
  3. Charlie Brown (charlie@example.com)

Application completed successfully!
```

## 🎨 Project Graph Visualization

Run `npx nx graph` to see an interactive visualization of how projects depend on each other. This helps understand the architecture and impact of changes.

---

**Happy coding!** 🦀✨
