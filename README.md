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
    -p, --preferences <PREFERENCES>    Filepath for json preferences. [default: default]

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
