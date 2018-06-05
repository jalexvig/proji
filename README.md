# proji

PROJect Initialization commandline utility.

### example use

```
proji my_new_project
```

### options

```
USAGE:
    proji [OPTIONS] <NAME>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --profile <PROFILE>    Filepath for json profile. [default: default]

ARGS:
    <NAME>    The directory to initialize project.
```

# installation

### build

```
git clone https://github.com/jalexvig/proji.git
cd proji
cargo build --release
```

### accessibility

You have a few options:

Alias 

```
echo 'alias proji="$PWD/target/release/proji"' >> ~/.bash_aliases
```

Or you can add proji to a directory on your PATH:

```
cp target/release/proji /path/to/a/directory/in/PATH
```

# profiles

You can pass a project configuration profile to `proji` using the `--profile` option. `proji` will look in your `$HOME/.proji` directory for the profile supplied.

Profiles are json files and specify a few things:

1. `name` -- the name of the author (this will appear in the license)
2. `license` -- the name of the license (e.g. "mit")
3. `commands` -- a list of commands to pass a shell instance once the new project directory is set up.

*default.json*
```json
{
  "name": "Alex",
  "license": "mit"
}
```

`proji` supports inheritance using `"inherits"` json key and supplying a list of profiles to inherit from:

*intellij.json*
```json
{
  "commands": [
    "echo '.idea' >> .git/info/exclude"
  ]
}
```

*python.json*
```json
{
  "inherits": ["intellij", "default"],
  "commands": [
    "echo 'venv' >> .gitignore",
    "python3 -m venv venv",
    "source venv/bin/activate"
  ]
}
```

A non-list attribute from a child will override the corresponding attribute from its parents. Commands from parents and children will both be run with parent commands running before children commands. In the case of multiple inheritance (see example above) [C3 linearization](https://en.wikipedia.org/wiki/C3_linearization) is used to figure out the order of inheritance. Commands from profiles further away in this linearization will be executed first.
