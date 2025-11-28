# Contributing to EarPlayer

Thank you for your interest in contributing to EarPlayer!

## Development Setup

1. **Prerequisites**
   - Rust 1.70+
   - ALSA development libraries (Linux)
   - BlueZ for BLE MIDI support

2. **Clone and Build**
   ```bash
   git clone https://github.com/doobidoo/EarPlayer.git
   cd EarPlayer/ear-trainer
   cargo build
   ```

3. **Run Tests**
   ```bash
   cargo test
   ```

## Semantic Versioning

This project follows [Semantic Versioning](https://semver.org/):

- **MAJOR** (x.0.0): Breaking changes
- **MINOR** (0.x.0): New features, backward compatible
- **PATCH** (0.0.x): Bug fixes, backward compatible

## Making a Release

### Option 1: GitHub Actions (Recommended)

1. Go to **Actions** → **Release** workflow
2. Click **Run workflow**
3. Select version bump type (patch/minor/major)
4. Optionally add release notes
5. The workflow will:
   - Calculate new version
   - Update VERSION, Cargo.toml, CHANGELOG.md
   - Create git tag
   - Push changes
   - Create GitHub release with binary

### Option 2: Local Script

```bash
./scripts/release.sh patch   # 0.2.0 → 0.2.1
./scripts/release.sh minor   # 0.2.0 → 0.3.0
./scripts/release.sh major   # 0.2.0 → 1.0.0
```

Then push:
```bash
git push origin main --tags
```

## Commit Message Format

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Formatting, missing semicolons, etc.
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `perf`: Performance improvement
- `test`: Adding tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(audio): add BLE MIDI support
fix(synth): eliminate audio crackling
docs: update README with BLE setup instructions
chore(release): v0.2.0
```

## Pull Request Process

1. Fork the repository
2. Create a feature branch: `git checkout -b feat/amazing-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Commit with conventional commit message
6. Push to your fork
7. Open a Pull Request

## Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and address warnings
- Add tests for new functionality
- Update documentation as needed

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
