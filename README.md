
# pinowm

very simple window manager written in rust to learn X11/xcb and also how a
window manager works under the hood.

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

- [ ] proper key codes from xmodmap
- [ ] borders
- [ ] statusbar (maybe as seperate crate)
- [ ] dynamic window tiling

