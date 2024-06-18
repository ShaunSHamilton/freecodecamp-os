# freecodecamp-os

## v4 Development

https://github.com/freeCodeCamp/freeCodeCampOS/issues/589

## Commands

Start docs server:

```bash
/$ mdbook serve
```

## Structure

- `/cli` cli
- `/client` client
  - `dist/` build
- `/config` config
- `/docs` docs
  - `dist/` build
- `/parser` parser
- `/server` server
- `/runner` runner

## Runner

Test runners for different languages. Supported languages:

- [ ] Nodejs
- [ ] Python
- [ ] Bash
- [ ] Rust
  - Code is wrapped in `fn <zero_collision_name>() { <code> }`

### Considerations

- Create a temp directory + files (project) to run tests
- How to handle `before-all`, `before-each`, `after-all`, `after-each` hooks?
  - Specifically, shared state/global
- Helpers?
  - Always import a `helpers` module?
- Each test is run within equivalent of `main` function as a `function`

1. Runner creates a temp directory with configured files.
1. Server gives test string to runner.
1. Runner writes test string to file with needed stuffs.

Permanent test dir for cached helpers/crates?

Force locations of test dir. Force location of helpers.

```console
.fcc-tests/
|- rust/
|  |- target/
|  |- src/
|  |  |- main.rs
|  |  |- helpers.rs
|  |- Cargo.toml
|- node/
|  |- node_modules/
|  |- src/
|  |  |- index.js
|  |  |- helpers.js
|  |- package.json
|- python/
|  |- src/
|  |  |- main.py
|  |  |- helpers.py
|- bash/
|  |- src/
|  |  |- main.sh
|  |  |- helpers.sh
```

### Example

```rust
let output = helpers::custom_func();
println!("{}", output);
```

Produces:

```rust
mod helpers;

fn main() {
  test_1();
}

fn test_1() {
  let output = helpers::custom_func();
  println!("{}", output);
}
```
