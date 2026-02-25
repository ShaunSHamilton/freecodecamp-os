# freecodecamp-os

## Structure

- `/cli` cli
- `/client` client
  - `dist/` build
- `/config` config
- `/docs` docs
  - `dist/` build
- `example`
- `/parser` parser
- `/server` server
- `/runner` runner

## `cli`

The CLI is used to set up new curricula, add/remove projects, and validate the config.

### TODO

- [ ] update deps
- [ ] add project creation command
- [ ] add project deletion command
- [ ] add config validation command

## `client`

The client is the main interface for users to read the curriculum content, and see test output/results.

### TODO

- [ ] update with latest tooling
  - ensure use of vite rolldown
  - update whole project to use `bun` instead of `npm`

## `config`

A library of types relating to the structure of `freecodecamp.conf.json` as well as common types shared between `cli`, `client`, `parser`, `server`, and `runner`.

### TODO

- [ ] update with full types needed to match feature parity with `https://github.com/freeCodeCamp/freeCodeCampOS.git`

## `docs`

MDBook docs for this application.

### TODO

- [ ] copy and update what is from `https://github.com/freeCodeCamp/freeCodeCampOS.git`

## `example`

A basic curriculum using `freecodecamp-os` for testing and on-boarding.

### TODO

- [ ] copy and update what is from `https://github.com/freeCodeCamp/freeCodeCampOS.git` in the `self` folder.

## `parser`

The parser handles the curriculum Markdown files - converting them into structures the server can use.

### TODO

- [ ] implement parser similar to `https://github.com/freeCodeCamp/freeCodeCampOS.git` in `.freeCodeCamp/tooling/parser.js`

## `server`

The server serves and communicates with the client, watches the filesystem for changes, and calls the test runner.

### TODO

- [ ] update with latest tooling
  - ensure use of axum
  - finish impl of websockets
- [ ] embedded client dist as static for distribution
- [ ] copy similar functionality from `https://github.com/freeCodeCamp/freeCodeCampOS.git` in `.freeCodeCamp/tooling/` folder
- [ ] use `runner`
- [ ] use `parser`

## `runner`

The runner is a library with functions to executes code, and handle file seeding.

### TODO

- [ ] add functions for seeding files
- [ ] add python runner
- [ ] add rust runner
