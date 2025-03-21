# CLAUDE.md - Guidelines for Ragzilla

## Build & Test Commands
- Build: `cargo build`
- Check: `cargo check`
- Format: `cargo fmt`
- Lint: `cargo clippy`
- Test all: `cargo test`
- Test single: `cargo test -p ragzilla-<crate> <test_name>`
- Test with output: `cargo test -- --nocapture`
- Run with env vars: `RUST_LOG=debug OPENAI_API_KEY=<key> cargo run`

## Code Style Guidelines
- **Imports**: Group by external then internal, alphabetize within groups
- **Error Handling**: Use Result with descriptive error types, propagate with `?`
- **Types**: Use strong typing, derive Debug for all structs, prefer owned over borrowed when appropriate
- **Naming**: snake_case for functions/variables, PascalCase for types, SCREAMING_SNAKE for constants
- **Documentation**: Document public APIs with examples and use cases
- **Testing**: Write both unit and integration tests, use descriptive names
- **API Keys**: Never hardcode API keys, load from environment variables using dotenvy
- **Formatting**: Follow rustfmt conventions, use 4 spaces for indentation