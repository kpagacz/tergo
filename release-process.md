# Dev guide

## Release process

### Rust libraries

The order of operations matters here.

1. Update versions in `Cargo.toml` files (library versions and dependencies):
   - aqua
   - spongia
   - unguentum
   - balnea
   - tergo

2. Make sure the documentation is updated (`README.md` files):
   - `balnea` - if there were any configuration changes they need
     to be reflected in the documentation.

3. Publish:
   - aqua
   - spongia
   - unguentum
   - balnea
   - tergo

### CLI tool

1. Update the versions of dependencies in `Cargo.toml`.
2. Update the documentation:
   - If the installation process changed, it needs to be reflected
     in the docs.
3. Publish.

### VSCode extension

1. Run `npx wit2ts --outDir ./src ./wit`
2. Run `vsce package`
3. Run `vsce publish` - this requires a Personal Access Token
   for `konradpagacz` organization.

### R package

1. Update the `tergo-lib` dependency in `antidotum/tergo/src/rust/Cargo.toml`
2. Run `extendr::document()`
3. Update the version in `DESCRIPTION`
4. Fill and update NEWS.md
5. R CMD build
6. R CMD check
7. Publish to CRAN

## Adding configuration

When adding a new configuration entry to `tergo`, make sure to update:

- the `Config` struct in `unguentum/src/config.rs`.
- the tests in `unguentum/tests/config_parsing.rs`.
- `get_config` and `get_default_config` functions in `antidotum/tergo/src/rust/src/lib.rs`.
- `README.md` in `balnea` if the configuration is user-facing.
- The vignette in `antidotum/tergo` if the configuration is user-facing.
