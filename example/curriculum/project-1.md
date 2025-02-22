# Learn freeCodeCampOS

```json
{
  "id": 0,
  "isIntegrated": false,
  "is_public": true,
  "runTestsOnWatch": true,
  "seedEveryLesson": false,
  "isResetEnabled": true,
  "numberofLessons": null,
  "blockingTests": null,
  "breakOnFailure": null
}
```

In this course, you will learn how to use the `@freecodecamp/freecodecamp-os` package to develop courses.

## 0

### --description--

Welcome to freeCodeCampOS! 👋

This example project will walk you through some of the features of freeCodeCampOS, and how to use them for your own course.

Start by opening the `curriculum/locales/english/learn-freecodecamp-os.md` file in your editor. Then, click the `Run Tests` button to go to the next lesson.

<details>
  <summary>Tidbit</summary>

Did you know the "OS" in freeCodeCampOS stands for "Open Source"?

![image](./images/image.png)

</details>

### --tests--

This is a test that will always pass.

```js,runner=node
console.log("Test");
assert(true);
```

## 1

### --description--

The `learn-freecodecamp-os.md` file is a markdown file that contains the content for your course. You will learn more about this later. For now, learn how to use the UI.

Click the `Run Tests` button again. Then, click the `Console` tab in the bottom panel, expand the test `details`, and follow the instructions.

### --tests--

This is a test that will always fail.

```js,runner=node
await new Promise((resolve) => setTimeout(resolve, 5000));
assert.fail(
  "This is a custom test assertion message. Click the > button to go to the next lesson"
);
```

## 2

### --description--

Click the `Run Tests` button to see two failed tests.

Then, change the sentence `Welcome to freeCodeCampOS!` in the `learn-freecodecamp-os.md` file to anything you want to see one test pass.

Finally, check the `Console` tab for further instructions.

### --tests--

You should edit the `Welcome to freeCodeCampOS!` sentence in the `curriculum/locales/english/learn-freecodecamp-os.md` file to anything you want.

```js,runner=node
const { readFile } = await import("fs/promises");
const file = await readFile(
  "curriculum/locales/english/learn-freecodecamp-os.md",
  "utf-8"
);
await new Promise((resolve) => setTimeout(resolve, 5000));
assert.notInclude(file.slice(0, 100), "Welcome to freeCodeCampOS!");
```

I always fail 🙃

```js,runner=node
await new Promise((resolve) => setTimeout(resolve, 3000));
console.log("Look! Worker stdout is printed in debug mode: ", __a);
assert(__a == 1);
assert.fail("Click the > button to go to the next lesson");
```

### --before-each--

#### --cmd--

```js,runner=node
const __a = 1;
```

## 3

### --description--

You changed something you should not have 😱, and you do not know how to continue.

Fret not! Press the `Reset Project` button to run the <dfn title="starting state for a lesson">seed</dfn>

### --tests--

The `curriculum/locales/english/learn-freecodecamp-os.md` file should contain the sentence `Welcome to freeCodeCampOS!`.

```js,runner=node
const { readFile } = await import("fs/promises");
const file = await readFile(
  join(ROOT, "curriculum/locales/english/learn-freecodecamp-os.md"),
  "utf-8"
);
assert.include(file.slice(0, 100), "Welcome to freeCodeCampOS!");
```

### --seed--

#### --cmd--

```bash,runner=bash
git restore curriculum/locales/english/learn-freecodecamp-os.md
```

## 4

### --description--

Now, on to creating your own course.

Open a new terminal, and cd into the `learn-freecodecamp-os/` directory.

### --tests--

You should be in the `learn-freecodecamp-os/` directory.

```js,runner=node
const cwd = await __helpers.getCWD();
assert.include(cwd, "learn-freecodecamp-os");
```

## 5

### --description--

Declare the `learn-freecodecamp-os/` directory as an npm project:

```bash,runner=bash
npm init -y
```

### --tests--

You should have a `package.json` file in `learn-freecodecamp-os/`.

```js,runner=node
const { access, constants } = await import("fs/promises");
try {
  await access(join(project.dashedName, "package.json"));
} catch (e) {
  assert.fail(e);
}
```

## 6

### --description--

Install `@freecodecamp/freecodecamp-os`.

### --tests--

You should have `@freecodecamp/freecodecamp-os` installed.

```js,runner=node
const { access, constants } = await import("fs/promises");
try {
  await access(
    join(project.dashedName, "node_modules/@freecodecamp/freecodecamp-os")
  );
} catch (e) {
  assert.fail(e);
}
```

Version `>=3` should be installed.

```js,runner=node
try {
  const { stdout, stderr } = await __helpers.getCommandOutput(
    "npm list",
    project.dashedName
  );
  assert.include(stdout, "@freecodecamp/freecodecamp-os@3");
} catch (e) {
  assert.fail(e);
}
```

### --hints--

#### --0--

Run `npm install @freecodecamp/freecodecamp-os` in the terminal

## 7

### --description--

Create a `config/` directory to hold your project and state config.

### --tests--

You should have a `config/` directory.

```js,runner=node
const { access, constants } = await import("fs/promises");
try {
  await access(join(project.dashedName, "config"));
} catch (e) {
  assert.fail(e);
}
```

## 8

### --description--

Create a `config/projects.json` file. Initialize it with `[]`.

### --tests--

You should have a `config/projects.json` file.

```js,runner=node
const { access, constants } = await import("fs/promises");
try {
  await access(join(project.dashedName, "config/projects.json"));
} catch (e) {
  assert.fail(e);
}
```

The `projects.json` file should contain `[]`.

```js,runner=node
const { readFile } = await import("fs/promises");
const file = await readFile(
  join(project.dashedName, "config/projects.json"),
  "utf-8"
);
assert.equal(file?.trim(), "[]");
```

## 9

### --description--

The mandatory properties for a project in the `projects.json` file are:

- `id`: a unique identifier for the project
- `dashedName`: a string of `-` separated words

Add the following to the `projects.json` file:

```json
{
  "id": 0,
  "dashedName": "learn-freecodecamp-os"
}
```

### --tests--

Your `projects.json` file should contain an array with one object.

```js,runner=node
assert.isArray(__projects);
assert.lengthOf(__projects, 1);
assert.isObject(__projects[0]);
```

The object should have the `id` and `dashedName` properties.

```js,runner=node
assert.hasAllKeys(__projects[0], ["id", "dashedName"]);
```

The `id` property should be `0`.

```js,runner=node
assert.equal(__projects[0].id, 0);
```

### --before-each--

#### --cmd--

```js,runner=node
const { readFile } = await import("fs/promises");
const file = await readFile(
  join(ROOT, project.dashedName, "config/projects.json"),
  "utf-8"
);
const __projects = JSON.parse(file);
```

## 10

### --description--

Every project defined in the `projects.json` file needs a directory in the root of the workspace with the same name as the project `dashedName`.

Create a directory within `learn-freecodecamp-os/` that matches the `dashedName` of the project in `learn-freecodecamp-os/config/projects.json`.

### --tests--

You should have a `learn-freecodecamp-os/learn-freecodecamp-os/` directory.

```js,runner=node
const { access, constants } = await import("fs/promises");
await access(join(project.dashedName, "learn-freecodecamp-os"));
```

## 11

### --description--

Create a `curriculum/locales/english/` directory to hold your course content.

<details>
  <summary>Note</summary>
  The reason for the directory format convention is to allow for multiple languages in the future.

For now, `english` is a required `locale`, and is used as the default.

</details>

### --tests--

You should have a `curriculum/` directory.

```js,runner=node
const { access, constants } = await import("fs/promises");
try {
  await access(join(project.dashedName, "curriculum"));
} catch (e) {
  assert.fail(e);
}
```

You should have a `curriculum/locales/` directory.

```js,runner=node
const { access, constants } = await import("fs/promises");
try {
  await access(join(project.dashedName, "curriculum/locales"));
} catch (e) {
  assert.fail(e);
}
```

You should have a `curriculum/locales/english/` directory.

```js,runner=node
const { access, constants } = await import("fs/promises");
try {
  await access(join(project.dashedName, "curriculum/locales/english"));
} catch (e) {
  assert.fail(e);
}
```

## 12

### --description--

Create a `curriculum/locales/english/learn-freecodecamp-os.md` file.

### --tests--

You should have a `curriculum/locales/english/learn-freecodecamp-os.md` file.

```js,runner=node
const { access, constants } = await import("fs/promises");
try {
  await access(
    join(
      project.dashedName,
      "curriculum/locales/english/learn-freecodecamp-os.md"
    )
  );
} catch (e) {
  assert.fail(e);
}
```

## 13

### --description--

Add a title to the `learn-freecodecamp-os.md` file.

```markdown
# freeCodeCampOS Title
```

### --tests--

The `learn-freecodecamp-os.md` file should contain a title.

```js,runner=node
const { readFile } = await import("fs/promises");
const file = await readFile(
  join(
    project.dashedName,
    "curriculum/locales/english/learn-freecodecamp-os.md"
  ),
  "utf-8"
);
assert(file.startsWith(), "# freeCodeCampOS Title");
```

## 14

### --description--

Add the first lesson to the `learn-freecodecamp-os.md` file, with a description heading:

```markdown
## 0

### --description--
```

### --tests--

The `learn-freecodecamp-os.md` file should contain a lesson.

```js,runner=node
const { readFile } = await import("fs/promises");
const file = await readFile(
  join(
    project.dashedName,
    "curriculum/locales/english/learn-freecodecamp-os.md"
  ),
  "utf-8"
);
assert(file.includes("\n## 0"));
```

The lesson should have a description heading.

```js,runner=node
const { readFile } = await import("fs/promises");
const file = await readFile(
  join(
    project.dashedName,
    "curriculum/locales/english/learn-freecodecamp-os.md"
  ),
  "utf-8"
);
assert(file.includes("\n### --description--"));
```

## 15

### --description--

Signify the end of the file, by adding the following:

```markdown
## --fcc-end--
```

### --tests--

The `learn-freecodecamp-os.md` file should contain the `--fcc-end--` marker.

```js,runner=node
const { readFile } = await import("fs/promises");
const file = await readFile(
  join(
    project.dashedName,
    "curriculum/locales/english/learn-freecodecamp-os.md"
  ),
  "utf-8"
);
assert(file.includes("\n## --fcc-end--"));
```

## 16

### --description--

Within `learn-freecodecamp-os/`, create a `freecodecamp.conf.json` file.

### --tests--

You should have a `freecodecamp.conf.json` file.

```js,runner=node
const { access, constants } = await import("fs/promises");
try {
  await access(join(project.dashedName, "freecodecamp.conf.json"));
} catch (e) {
  assert.fail(e);
}
```

## 17

### --description--

Within the `freecodecamp.conf.json` file, add the following:

```json
{
  "version": "0.0.1",
  "config": {
    "projects.json": "<PROJECTS_JSON>",
    "state.json": "<STATE_JSON>"
  },
  "curriculum": {
    "locales": {
      "<LOCALE>": "<LOCALE_DIR>"
    }
  }
}
```

### --tests--

The `freecodecamp.conf.json` file should contain the `version` property.

```js,runner=node
assert.hasAllKeys(__conf, ["version"]);
```

The `version` property should be `0.0.1`.

```js,runner=node
assert.equal(__conf.version, "0.0.1");
```

The `freecodecamp.conf.json` file should contain the `scripts` property.

```js,runner=node
assert.hasAllKeys(__conf, ["scripts"]);
```

The `freecodecamp.conf.json` file should contain the `config` property.

```js,runner=node
assert.hasAllKeys(__conf, ["config"]);
```

The `config` property should be an object.

```js,runner=node
assert.isObject(__conf.config);
```

The `config` property should contain the `projects.json` property.

```js,runner=node
assert.hasAllKeys(__conf.config, ["projects.json"]);
```

The `projects.json` property should be a string.

```js,runner=node
assert.isString(__conf.config["projects.json"]);
```

The `config` property should contain the `state.json` property.

```js,runner=node
assert.hasAllKeys(__conf.config, ["state.json"]);
```

The `state.json` property should be a string.

```js,runner=node
assert.isString(__conf.config["state.json"]);
```

The `freecodecamp.conf.json` file should contain the `curriculum` property.

```js,runner=node
assert.hasAllKeys(__conf, ["curriculum"]);
```

The `curriculum` property should be an object.

```js,runner=node
assert.isObject(__conf.curriculum);
```

The `curriculum` property should contain the `locales` property.

```js,runner=node
assert.hasAllKeys(__conf.curriculum, ["locales"]);
```

The `locales` property should be an object.

```js,runner=node
assert.isObject(__conf.curriculum.locales);
```

The `locales` property should contain the `<LOCALE>` property.

```js,runner=node
assert.hasAllKeys(__conf.curriculum.locales, ["<LOCALE>"]);
```

The `<LOCALE>` property should be a string.

```js,runner=node
assert.isString(__conf.curriculum.locales["<LOCALE>"]);
```

The `locales` property should contain the `<LOCALE_DIR>` property.

```js,runner=node
assert.hasAllKeys(__conf.curriculum.locales, ["<LOCALE_DIR>"]);
```

The `<LOCALE_DIR>` property should be a string.

```js,runner=node
assert.isString(__conf.curriculum.locales["<LOCALE_DIR>"]);
```

### --before-each--

#### --cmd--

```js,runner=node
const { readFile } = await import("fs/promises");
const conf = await readFile(
  join(project.dashedName, "freecodecamp.conf.json"),
  "utf-8"
);
const __conf = JSON.parse(conf);
```

## 18

### --description--

Within the `freecodecamp.conf.json` file, replace the `<PROJECTS_JSON>` placeholder with the relative path to the `projects.json` file. _Relative to your courses root_.

### --tests--

The `projects.json` property should be a relative path to the `projects.json` file.

```js,runner=node
assert.equal(__conf.config["projects.json"], "config/projects.json");
```

## 19

### --description--

Within the `freecodecamp.conf.json` file, replace the `<STATE_JSON>` placeholder with the relative path to the `state.json` file. _Relative to your courses root_.

### --tests--

The `state.json` property should be a relative path to the `state.json` file.

```js,runner=node
assert.equal(__conf.config["state.json"], "config/state.json");
```

## 20

### --description--

Within the `freecodecamp.conf.json` file, replace the `<LOCALE>` placeholder with `english`. Then, replace the `<LOCALE_DIR>` placeholder with `curriculum/locales/english/`.

**Note:** Currently, `english` is a required locale, and is used as the default.

### --tests--

The `<LOCALE>` property should point to the locale of your course.

```js,runner=node
assert.include(
  __conf.curriculum.locales["english"],
  "curriculum/locales/english"
);
```

## 21

### --description--

Those are all the pre-requisites to start the development server. Within the `learn-freecodecamp-os/` directory, run:

```bash
NODE_ENV=development node ./node_modules/@freecodecamp/freecodecamp-os/.freeCodeCamp/tooling/server.js
```

### --tests--

The development server should be running.

```js,runner=node
await fetch("http://localhost:8080");
```

## 22

### --description--

The development server runs at the port number defined by the `port` field in the `freecodecamp.conf.json` file, but defaults to `8080`. Open `http://localhost:8080` in your browser to see the course.

Also, take a look at the terminal output; in development, your config is validated - errors and warnings are logged to the terminal if something is not quite right.

To move on, click the `Run Tests` button.

### --hints--

#### --0--

In VSCode, you can open a webpage within the editor by:

- Clicking `Ctrl/Cmd + Shift + P`
- Typing and selecting `Simple Browser: Show`
- Inputing the localhost URL

#### --1--

Notice this course teaching you how to create a course is itself a course, and is running on port `8080`.

### --tests--

This test always passes.

```js,runner=node
assert(true);
```

## 23

### --description--

The terminal should have a warning about the first lesson description being empty.

Fix this by adding the following text:

```markdown
Welcome to freeCodeCampOS! 👋

This example project will walk you through some of the features of freeCodeCampOS, and how to use them for your own course.

Start by opening the `curriculum/locales/english/learn-freecodecamp-os.md` file in your editor. Then, click the `Run Tests` button to go to the next lesson.

<details>
  <summary>Tidbit</summary>

Did you know the "OS" in freeCodeCampOS stands for "Open Source"?

</details>
```

### --hints--

#### --0--

Notice the description can accept any text, and will parse it as GFM (GitHub Flavored Markdown).

### --tests--

You should add the provided text to the `learn-freecodecamp-os.md` file.

```js,runner=node
const { readFile } = await import("fs/promises");
const file = await readFile(
  join(
    project.dashedName,
    "curriculum/locales/english/learn-freecodecamp-os.md"
  ),
  "utf-8"
);
assert.include(
  file,
  `Welcome to freeCodeCampOS! 👋

This example project will walk you through some of the features of freeCodeCampOS, and how to use them for your own course.

Start by opening the \`curriculum/locales/english/learn-freecodecamp-os.md\` file in your editor. Then, click the \`Run Tests\` button to go to the next lesson.

<details>
  <summary>Tidbit</summary>

Did you know the "OS" in freeCodeCampOS stands for "Open Source"?

</details>`
);
```

## 24

### --description--

Also, there should be a warning about the first lesson not having any tests.

Add a test by placing the 3rd-level heading `### --tests--` within the 2nd-level heading `## 0`:

````markdown
### --tests--

This is a test that will always fail.

```js,runner=node
assert.fail(
  "This is a custom test assertion message. Click the > button to go to the next lesson"
);
```
````

### --hints--

#### --0--

Tests take the form:

````markdown
### --tests--

<TEST_TEXT>

```js,runner=node
<TEST_CODE>
```

<SECOND_TEST_TEXT>

```js,runner=node
<TEST_CODE>
```
````

#### --1--

The test code is evaluted in a Nodejs context. So, any Nodejs code is valid.

#### --2--

Notice the use of `assert.fail` in the test code. There are many globals available to you in the test code.

Read the docs to learn more.

### --tests--

You should add the provided test to the `learn-freecodecamp-os.md` file.

```js,runner=node
const { readFile } = await import("fs/promises");
const file = await readFile(
  join(
    project.dashedName,
    "curriculum/locales/english/learn-freecodecamp-os.md"
  ),
  "utf-8"
);
assert.include(
  file,
  `### --tests--

This is a test that will always fail.

\`\`\`js
assert.fail(
  'This is a custom test assertion message. Click the > button to go to the next lesson'
);
\`\`\``
);
```

## 25

### --description--

To run the tests, you could click the `Run Tests` button again, but there is a better way. A project can be configured to run tests on file change with the `runTestsOnWatch` flag.

Add `"runTestsOnWatch": true` to the project in the `projects.json` file.

### --tests--

The `projects.json` file should contain the `runTestsOnWatch` property.

```js,runner=node
assert.hasAllKeys(__projects[0], ["runTestsOnWatch"]);
```

The `runTestsOnWatch` property should have a value of `true`.

```js,runner=node
assert.isTrue(__projects[0].runTestsOnWatch);
```

### --before-each--

#### --cmd--

```js,runner=node
const { readFile } = await import("fs/promises");
const file = await readFile(
  join(project.dashedName, "config/projects.json"),
  "utf-8"
);
const __projects = JSON.parse(file);
```

## 26

### --description--

**Summary**

You have learnt how to:

- [x] install freecodecamp-os
- [x] add required files
- use the Markdown syntax to:
  - [x] add a title
  - [x] add a lesson
  - [x] add a description
  - [x] add tests
  - [ ] add seed
  - [ ] add hints
- [ ] use the `tooling` feature
- [ ] use the reset feature
- [ ] use the `terminal` feature
- [ ] use the `static` feature
- [ ] use the various project flags:
  - [ ] `isPublic`
  - [ ] `isIntegrated`
  - [ ] `blockingTests`
  - [ ] `breakOnFailure`
  - [x] `runTestsOnWatch`
  - [ ] `seedEveryLesson`
  - [ ] `isResetEnabled`
- [ ] ignore directories for the hot-reload feature

### --tests--

When you are done, type `done` in the terminal.

```js,runner=node
const lastCommand = await __helpers.getLastCommand();
assert.include(lastCommand, "done");
```
