# Contributing to NewsAPI Rust Client

First off, thank you for considering contributing to the NewsAPI Rust client! It's people like you that make this library better for everyone.

## Getting Started

Before you begin:
- Make sure you have a [GitHub account](https://github.com/signup/free)
- Ensure you have Rust installed (preferably via [rustup](https://rustup.rs/))
- Familiarize yourself with the [NewsAPI documentation](https://newsapi.org/docs)

## Setting Up Your Development Environment

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```
   git clone https://github.com/YOUR-USERNAME/newsapi-rs.git
   cd newsapi-rs
   ```
3. Configure your environment:
   ```
   cp .env.example .env
   ```
4. Edit `.env` and add your NewsAPI key:
   ```
   NEWSAPI_API_KEY=your_api_key_here
   ```
5. Run tests to verify your setup:
   ```
   cargo test
   ```

## Making Changes

1. Create a new branch for your changes:
   ```
   git checkout -b feature/your-feature-name
   ```
2. Make your changes
3. Run tests and ensure they pass:
   ```
   cargo test
   cargo fmt -- --check
   cargo clippy
   ```
4. Update documentation as needed
5. Commit your changes using clear commit messages

## Code Style

- Follow Rust's official style guide
- Use `cargo fmt` to format your code
- Run `cargo clippy` and address any linting issues
- Keep functions small and focused on a single responsibility
- Add tests for new features

## Documentation

- Update the README.md if necessary
- Add inline documentation using Rust's doc comments (`///`)
- Document all public API functions, types, and traits
- Include examples in documentation where appropriate

## Adding New Features

When adding new features:

1. Start by adding tests that would pass if the feature worked
2. Implement the minimal code required to make tests pass
3. Update documentation to reflect the new feature
4. Ensure the feature is consistent with the existing API design

## Submitting Changes

1. Push your changes to your fork
2. Submit a pull request from your branch to the main repository
3. In the pull request description, explain your changes and reference any related issues

## Pull Request Process

1. Update the README.md and documentation with details of changes
2. Increase version numbers in Cargo.toml and other files following [SemVer](http://semver.org/)
3. The pull request will be merged once it passes CI and is reviewed by maintainers

## Publishing Releases

For maintainers: To publish a new version to crates.io:

1. Update the version in `Cargo.toml` following [SemVer](http://semver.org/)
2. Commit the version change and create a git tag:
   ```
   git tag -a v0.1.1 -m "Release version 0.1.1"
   git push origin v0.1.1
   ```
3. The GitHub Actions workflow will automatically:
   - Verify the tag version matches `Cargo.toml`
   - Run all tests
   - Publish to crates.io using the `CARGO_REGISTRY_TOKEN` secret

Note: The `CARGO_REGISTRY_TOKEN` secret must be configured in the repository settings with a valid crates.io API token.

## Testing

- Write tests for all new features and bug fixes
- Make sure your tests cover both success and failure cases
- Use mock servers for testing API interactions where appropriate
- Include both unit and integration tests

## License

By contributing, you agree that your contributions will be licensed under the project's MIT License.

## Questions?

If you have questions about contributing, feel free to:
- Open an issue with your question
- Ask in the pull request
