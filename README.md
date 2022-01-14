# envm

A command line tool help to manage the environment file.

## Setup

Run the following command to generate envm repository at current directory.

```bash
envm init
```

The command will generate a .envm directory at current directory, and contains `config` and `HEAD` file.

### Config

The config file should look like as follow.

```toml
local = ".env"
pattern = ".env.{}"
template = ".env.example"
```

`local` is the file your progame used to read the environment variables, for example the docker compose will read from `.env`.

`pattern` is the pattern of environment file, for example if we have two environment file `.env.dev` and `.env.production`, the pattern will be `.env.{}`, the `{}` will used to replace by environment name.

`template` is the template environment file, use to generate other environment file.


