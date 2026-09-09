# Development

To start this project, you will need the following installed on your computer:

| Dependency                                           | Use                 |
| ---------------------------------------------------- | ------------------- |
| [moon](https://moonrepo.dev/docs/install#installing) | Monorepo management |
| [deno](https://deno.com)                             | JavaScript runtime  |

## Running tasks

**Tasks** can be ran using `moon run` followed by the name of the taks to run (multiple tasks can be ran at the same time). The tasks are defined within the projects which paths are specified in [`.moon/workspace.yml`](./.moon/workspace.yml) under the `projects` key. Within these folders, a `moon.yml` file contains the configuration for that given project. Within it a `tasks` key contains all the tasks that are available for that project.

To run a single task, use:

```bash
moon run <project>:<task>
```

For example, building the desktop application can be done using:

```bash
moon run desktop:build
```

## Commit messages

This repo is set up with [commitlint](https://commitlint.js.org) and a git hook in [`.moon/workspace.yml`](./.moon/workspace.yml) to ensure that commit messages follow the same commit convention. Every time a commit is made, it must start with one of these types:

| Type      | Meaning                                                  |
| --------- | -------------------------------------------------------- |
| `build:`  | Change affecting build system                            |
| `ci:`     | Changed something related to continuous integration (CI) |
| `chore:`  | Non-production changes                                   |
| `docs:`   | Added or modified documentation                          |
| `feat:`   | Added new feature                                        |
| `fix:`    | Fixing a bug                                             |
| `revert:` | Reverting a commit                                       |
| `perf:`   | Improved performance                                     |
| `style:`  | Changed structure or style of code                       |
| `test:`   | Added or modified tests                                  |

This type is then followed by a space, with the commit message starting with a lower-case letter.

## Hooks

To synchronize hooks across the repository use `moon sync hooks`. This has to be done if a hook is created or if an existing hook has been modified.

## Database

The database can be ran using `docker compose up -d` while having Docker Desktop open. The database's contents can be visible via the PgAdmin RDBMS at [`http://localhost:5050`](http://localhost:5050). The informations to log in to PgAdmin are around in `.env` with the values `PGADMIN_DEFAULT_EMAIL` and `PGADMIN_DEFAULT_PASSWORD`.

Then connecting the database can be done with the remaining few elements to match when creating a server.
