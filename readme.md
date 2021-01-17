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
    }
}
```

To move a file, simply typed

```code
emv down hello.txt docs
```

This will move the hello.txt in ~/Downloads to ~/Documents
