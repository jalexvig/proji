# proji

PROJect Initialization commandline utility. Initially this is setup for Python3.

### example use

```
proji project_name
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
