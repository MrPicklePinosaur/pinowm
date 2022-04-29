
# pinowm

very simple window manager written in rust to learn X11/xcb and also how a
window manager works under the hood.

some features include
- dynamic tiling
- status bar (included as seperate crate)
- config file

## RUNNING FOR DEVELOPMENT

to run with `startx` include the line
```
exec cargo run --manifest-path [PATH TO Cargo.toml]
```
in your `.xinitrc`

then just run
```
$ startx
```

## TODO

- [x] proper key codes from xmodmap
- [x] borders
- [ ] statusbar (maybe as seperate crate)
- [x] dynamic window tiling
- [ ] window focus + change focus
- [ ] proper hotkey config system

