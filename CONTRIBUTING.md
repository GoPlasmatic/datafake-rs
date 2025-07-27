# Contributing to datafake-rs

Thank you for your interest in contributing to datafake-rs! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

By participating in this project, you agree to abide by our code of conduct: be respectful, inclusive, and constructive in all interactions.

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/your-username/datafake-rs.git
   cd datafake-rs
   ```
3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/GoPlasmatic/datafake-rs.git
   ```

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building the Project

```bash
cargo build
cargo test
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run doc tests
cargo test --doc
```

### Code Quality

Before submitting a PR, ensure your code passes all checks:

```bash
# Format code
cargo fmt

# Run clippy
cargo clippy -- -D warnings

# Check documentation
cargo doc --no-deps --all-features
```

## Making Changes

1. Create a new branch for your feature or bugfix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes, following the coding standards below

3. Add tests for your changes

4. Ensure all tests pass and code quality checks succeed

5. Commit your changes with a clear commit message:
   ```bash
   git commit -m "feat: add new fake data type for X"
   ```

## Coding Standards

### Rust Style

- Follow standard Rust naming conventions
- Use `rustfmt` for consistent formatting
- Address all `clippy` warnings
- Write idiomatic Rust code

### Documentation

- Add documentation comments (`///`) for all public APIs
- Include examples in documentation where appropriate
- Update README.md if adding new features
- Keep CLAUDE.md updated with architectural changes

### Testing

- Write unit tests for new functionality
- Add integration tests for new fake data types
- Ensure tests are deterministic (use seeded RNG when needed)
- Test edge cases and error conditions

### Commit Messages

Follow the conventional commits specification:

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `test:` Test additions or modifications
- `refactor:` Code refactoring
- `perf:` Performance improvements
- `chore:` Maintenance tasks

## Adding New Fake Data Types

When adding a new fake data type:

1. Add the implementation in `src/operators/fake.rs`
2. Import necessary fake-rs modules
3. Add a new match arm in the `generate` method
4. Add unit tests for the new type
5. Update the README.md with the new type in the appropriate section
6. Add an example in the documentation

Example:
```rust
// In src/operators/fake.rs
"new_type" => Ok(Value::String(NewType().fake())),
```

## Pull Request Process

1. Update your branch with the latest upstream changes:
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. Push your branch to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

3. Create a Pull Request on GitHub

4. Fill out the PR template with:
   - Description of changes
   - Related issue (if any)
   - Testing performed
   - Breaking changes (if any)

5. Wait for review and address any feedback

## Review Process

- All PRs require at least one review before merging
- CI must pass (tests, formatting, clippy)
- Documentation must be updated if needed
- Breaking changes must be clearly documented

## Release Process

Releases are automated through GitHub Actions when a new tag is pushed:

```bash
git tag v0.2.0
git push origin v0.2.0
```

## Getting Help

If you need help:

1. Check existing issues and discussions
2. Review the documentation
3. Open a new issue with a clear description

## Recognition

Contributors will be recognized in:
- The project's contributors list
- Release notes for significant contributions

Thank you for contributing to datafake-rs!