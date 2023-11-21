# Installation

## For gschemas

### For GNU/Linux or macOS machines:
- Copy the schema and compile schemas
```
mkdir -p $HOME/.local/share/glib-2.0/schemas
cp data/schemas/com.prophesy.gschema.xml $HOME/.local/share/glib-2.0/schemas
glib-compile-schemas $HOME/.local/share/glib-2.0/schemas/
```
- Append the directory of glib2.0 to XDG_DATA_DIRS environment variables.
Eg: If schemas are present under  ~/.local/share/glib-2.0/schemas/

    `export XDG_DATA_DIRS=$XDG_DATA_DIRS:$HOME/.local/share/`

### For Windows machines (Untested):
- Copy the schema and compile schemas
```
mkdir C:/ProgramData/glib-2.0/schemas/
cp data/schemas/com.prophesy.gschema.xml C:/ProgramData/glib-2.0/schemas/
glib-compile-schemas C:/ProgramData/glib-2.0/schemas/
```

Additionally, gtk4 libraries are also required.

Note: For svg icons librsvg needs to be installed. (tested via gvsbuild)</b>

# Development

## Dependencies
gtk4-devel is required. For more details follow [gtk-rs book](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation.html)

## Checking for errors
`cargo check`

## Building
Debug: `cargo build`

Release: `cargo build --release`

## Running

### With cargo
`cargo run`

### vscode
Configurations are present in .vscode/launch.json .

# References
- [gtk-rs](https://gtk-rs.org/)
- [gtk-rs-book](https://gtk-rs.org/gtk4-rs/stable/latest/book/)
- [GNOME Developer Documentation](https://developer.gnome.org/documentation/introduction.html)