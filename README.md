# freecodecamp-os

## v4 Development

https://github.com/freeCodeCamp/freeCodeCampOS/issues/589

## Commands

Start docs server:

```bash
/$ mdbook serve
```

## Structure

- `/app` desktop application
  - `bins/` external binaries included in app
  - `dist/` frontend build
  - `src/` frontend source
  - `src-tauri/` backend source
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

Each test is given a runner name following:

````markdown
```<code_lang>,runner=<runner_name>
<test>
```

```<code_lang>
<test>
```
````

Where, if no runner name is specified, the Markdown codeblock language is used as the runner name.

A Runner is no more than a command that is passed a JSON-serializable string of the test meta - hooks, helpers, test code.

The runner name is mapped in the `freecodecamp.conf.json`:

```json
{
  "runners": {
    "rust": "runner rust",
    "js": "node --eval",
    "python": "runner python"
  }
}
```

Whether or not the runner is run in parallel or not is determined by the project configuration.

By default, if no matching `runners` key is found for a runner name, the included `runner` binary is tried. If the test runner is not available in the `runner` binary, the test is skipped, and a warning is logged.

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

---

Config is mandatory for using bin. Runner only needs part of config (passed by bin), and parser only needs part of the config (passed by bin).

Config is made of pieces defined by runner and parser (+ more).

Config probably should not be a public library.

---

## Routes

- `GET /`

Landing page with list of projects.

- `GET /<PROJECT>/<LESSON_ID>`

Example: `/0/0`

Gets the Markdown file for the lesson data.

- `POST /config { locale: <LOCAL> }`

- `POST /reset-lesson?project_id=<PROJECT_ID>&lesson_id=<LESSON_ID>`
- `POST /reset-project?project_id=<PROJECT_ID>`

## Websockets

- `CONNECT`
- `RUN_TESTS`
- `UPDATE_TESTS`
- `UPDATE_CONSOLE`
- `UPDATE_HINTS`
- `CANCEL_TESTS`
- `RESPONSE`

## TODO

- Frontend
  - [ ] Improve dev setup to handle reloads whilst keeping state
  - [ ] Websockets for tests
    - Create connection on `/tests/run` request, and kill it once all tests complete
    - Add `/tests/cancel` to websockets?
- Backend
  - [ ] Before/After hooks
