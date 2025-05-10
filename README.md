## A CLI tool to generate colors for dotfiles
Can be used to generate colors for multiple config files from a single color palette file.
<br>Currently only supports Xresources and Alacritty.

```console
Usage: colo [OPTIONS]

Options:
      --outputpath <OUTPUTPATH>  Output folder for generated files.
                                 If not set, generates a folder called 'output' in the current directory
      --colorpath <COLORPATH>    Path to a file containing color codes.
                                 If not set, assumes the file is in the current directory and is called 'colors.txt'
      --suffix <SUFFIX>           A string to be added to the end of filename. Default is no suffix
  -h, --help                     Print help
  -V, --version                  Print version
```

#### Example: generated Alacritty colors
```toml
[colors.primary]
background = "#1c1e26"
foreground = "#b5b7bd"
[colors.normal]
black = "#1c1e26"
red = "#876170"
green = "#618762"
yellow = "#6d5f69"
blue = "#414658"
magenta = "#6c6f93"
cyan = "#618780"
white = "#b5b7bd"
[colors.bright]
black = "#1c1e26"
red = "#876170"
green = "#618762"
yellow = "#6d5f69"
blue = "#414658"
magenta = "#6c6f93"
cyan = "#618780"
white = "#b5b7bd"
```