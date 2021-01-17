# Easy Move

Easily move files from one path to another.

NOTE: This project is still in progress.

## Usage

All path aliases are stored in a config file in $HOME/.easy-move/config.json

A valid config.json looks similar to this

```json
{
    path-aliases: {
        "~/Downloads":"down",
        "~/Documents":"docs",
    },
    resolution-prefix: "emv",
    attempt-resolution: true,
}
```

To move a file, simply type

```code
emv down hello.txt docs
```

This will move the hello.txt in ~/Downloads to ~/Documents.

If ```attempt-resolution: true``` this will add the prefix specified in ```resolution-prefix``` at the beginning of the filename in case the file exists in target directory.

We can also override the default behavior of resolution. To override the file in the target directory if it already exists use

```code
emv down hello.txt docs --force
```
