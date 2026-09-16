# Contributing to kyanite-core

First off, thanks for taking the time to contribute.

When contributing to this repository, please first discuss the change you wish to make via an issue, discussion, email, or any other method with the owners or contributors before making a change and opening a pull request.

Please note we have a [Code of Conduct](./CODE_OF_CONDUCT.md). Follow it in all your interactions with the project.

## Getting Started with Development

### Prerequisites

- **Rust Lang**: [rust-lang.org/tools/install](https://rust-lang.org/tools/install/) (rustup recommended)
- `rustfmt` and `clippy` components.
- A terminal and your editor of choice (VS Code, RustRover, Zed, etc.)

### Build & Verify

```bash
cargo build
cargo test --release --all-features

# Perform all checks using this script
check.sh # Only works on linux
```

Run the check shell script before pushing any changes.<br>
They perform linting, formatting and run tests/docs tests.

## Code Style & Conventions

### EditorConfig

An `.editorconfig` file is at the repo root. Most editors pick it up automatically. If yours does not, configure it to match the project's indentation and style rules.

### Naming Conventions

| Convention             | Applies to                                                  | Example                                |
|------------------------|-------------------------------------------------------------|----------------------------------------|
| `UpperCamelCase`       | Types, traits, enum variants, type parameters               | `MyType`, `MyTrait`, `MyVariant`, `T`  |
| `snake_case`           | Functions, methods, fields, params, locals, modules, crates | `my_function`, `my_field`, `my_module` |
| `SCREAMING_SNAKE_CASE` | Constants, statics                                          | `MY_CONST`                             |
| `'lowercase`           | Lifetimes                                                   | `'a`, `'de`                            |

### Code Quality

- No warnings on build. Run `cargo build` before pushing.
- Run formatting checks with `cargo fmt --all`.
- Run Clippy with `cargo clippy --allow-dirty --all-targets --all-features -- -D warnings` when available.
- Follow existing patterns in the codebase.
- Pay attention to static code analysis and code coverage.

## Testing

When adding tests:

- Follow current applied patterns.
- Use Rust’s built‑in test framework (`#[test]`).
- Place **unit tests** in the same module under `#[cfg(test)] mod tests`.
- Place **integration tests** in the `tests/` directory.
- Run all tests locally with `cargo test`.
- Run release tests with `cargo test --release --all-features`.
- Write tests for new code, including edge cases (non‑happy path).
- If test filtering is needed, use `cargo test <filter>`.
- Keep or improve test coverage if a coverage tool is configured.

Someone expand this section 🥀

## Commit Message Standards

### Conventional Commits

We follow [Conventional Commits](https://www.conventionalcommits.org/). This keeps history readable and enables automated changelog generation.

```
<type>(<scope>): <description>

[optional body]
[optional footer]
```

### Types

| Type       | Usage                                          |
|------------|------------------------------------------------|
| `feat`     | A new feature                                  |
| `fix`      | A bug fix                                      |
| `chore`    | Maintenance, tooling, config changes           |
| `docs`     | Documentation changes                          |
| `test`     | Adding or updating tests                       |
| `refactor` | Code restructuring without feature or fix      |
| `perf`     | Performance improvement without feature or fix |
| `style`    | Formatting, linting, code style (not CSS)      |
| `ci`       | CI/CD workflow changes                         |

### Scopes

Use a scope that indicates which part of the project the commit touches. Be as specific as makes sense:

| Scope      | When to use                         |
|------------|-------------------------------------|
| `core`     | Core abstractions or shared logic   |
| `api`      | Public API surface                  |
| `services` | Platform or service implementations |
| `models`   | Data models and records             |
| `readme`   | Changes in readme                   |

Examples:

```
feat(api): add Resource trait with usage getters
chore(ci): update Rust toolchain in workflow
docs(readme): improve quick start guide
```

### Signed Commits

All commits **must** be signed (GPG or SSH). Unsigned commits will not be accepted.

#### Minimal Git Setup

If you have not configured signing yet:

```bash
# Set your identity (must match your commit signature)
git config --global user.name "Your Name"
git config --global user.email "your-email@example.com"

# Get your secret key id
gpg --list-secret-keys

# Tell Git which signing key to use
git config --global user.signingkey <key-id>

# Enable signing for all commits
git config --global commit.gpgsign true
```

> [!IMPORTANT]
> For full guides on generating GPG keys and linking them to GitHub, see:
> - [GitHub: Managing commit signature verification](https://docs.github.com/en/authentication/managing-commit-signature-verification)
> - [Git Tools — Signing Your Work](https://git-scm.com/book/en/v2/Git-Tools-Signing-Your-Work)

### Atomic Commits

Each commit should represent **one logical change**. If a commit includes a bug fix, a refactor, and a documentation update, split it into multiple commits.

## Opening Issues

### Issue Templates

We provide templates for most scenarios. Select the one that best fits your issue when creating it in the issue tracker.

| Template        | Use when...                         |
|-----------------|-------------------------------------|
| Bug Report      | Something does not work as expected |
| Security Report | You found a vulnerability           |

Read the template instructions carefully before submitting.

### Security Disclosures

If your report could disclose a vulnerability or sensitive information, **do not** open a public issue.<br>
Instead, email the project maintainers at [akeoot@pm.me](mailto:akeoot@pm.me). See [SECURITY.md](./SECURITY.md).

## Making Pull Requests

### Before You Open

- [ ] Build completes without warnings (`cargo build`)
- [ ] Formatting passes (`cargo fmt --all -- --check`)
- [ ] Clippy passes (`cargo clippy --all-targets --all-features -- -D warnings`) when configured
- [ ] All tests pass (`cargo test --release --all-features`)
- [ ] New code includes tests (if applicable / when a test suite exists)
- [ ] Commits are signed
- [ ] Documentation is updated (if applicable)
- [ ] Branch is up to date with `main`

### Branch Naming

Use a prefix that matches the type of change, followed by a short descriptor:

```
feature/add-resource-interface
fix/service-null-ref
docs/improve-contributing-guide
chore/update-dependencies
```

Prefixes: `feature/`, `fix/`, `docs/`, `chore/`, `refactor/`, `test/`

### Pull Request Process

1. Ensure your branch is up to date with `origin/main`:
    ```bash
    git checkout main
    git pull origin main
    git checkout your-feature-branch
    git merge main   # rebase is disallowed
    ```
2. Open the PR against the `main` branch.
3. Fill out the pull request template.
4. Ensure all CI checks pass.
5. Request a review from a project maintainer.

### Review Process

- Pull requests require approval from project maintainers.
- All reviews are strict, changes that do not meet the standards above will be asked to improve before merging.

### Merge Strategy

**Squash merge** is preferred.<br>
This keeps the commit history clean by collapsing multiple commits into one logical commit per PR. The squashed commit message should follow the Conventional Commits format.

---

**Thank you for contributing to kyanite-core!**
