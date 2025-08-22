# CRUSH Development Guidelines

## Build Commands

### Rust Desktop App (scord-desktop)
- Build: `cd scord-desktop && cargo build`
- Run: `cd scord-desktop && cargo run`
- Release build: `cd scord-desktop && cargo build --release`

### Web App
- Dev server: `cd webapp && npm run dev`
- Build: `cd webapp && npm run build`
- Preview production build: `cd webapp && npm run preview`

## Lint Commands

### Rust
- Check: `cd scord-desktop && cargo check`
- Lint: `cd scord-desktop && cargo clippy`
- Format check: `cd scord-desktop && cargo fmt -- --check`

### TypeScript/Web
- Lint: `cd webapp && npm run lint`
- Type check: `cd webapp && tsc --noEmit`

## Test Commands

### Rust
- Run all tests: `cd scord-desktop && cargo test`
- Run specific test: `cd scord-desktop && cargo test test_name`
- Run tests with output: `cd scord-desktop && cargo test -- --nocapture`

### Web
- Run tests: `cd webapp && npm test` (if configured)
- Type check (as test): `cd webapp && tsc --noEmit`

## Code Style Guidelines

### Rust
- Use `cargo fmt` for formatting
- Follow Rust naming conventions (snake_case for variables/functions, PascalCase for types/traits)
- Use explicit error handling with `Result<T, E>` and `?` operator
- Group imports: standard library, external crates, local modules
- Use `clippy` for linting
- Prefer `expect` with descriptive messages over `unwrap`
- Use `const` for compile-time constants, `static` for mutable globals

### TypeScript/JavaScript
- Use TypeScript with strict typing
- Follow functional programming patterns where possible
- Use React functional components with hooks
- Use descriptive variable and function names (camelCase)
- Group imports: external libraries, internal modules
- Prefer `const` over `let` when possible
- Use async/await for asynchronous operations
- Use proper error boundaries and error handling
- Follow ESLint/Prettier configurations in the project

## Project Structure
- Rust app: `scord-desktop/` directory
- Web app: `webapp/` directory
- Shared data models should be synchronized between both implementations

## Git Workflow
- Create feature branches from main
- Write clear, concise commit messages
- Keep commits focused on a single change
- Update this file when adding new build/lint/test commands