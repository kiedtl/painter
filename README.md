# painter [![Build Status](https://travis-ci.org/lptstr/painter.svg?branch=master)](https://travis-ci.org/lptstr/painter)
Manage the Windows desktop wallpaper from the terminal

## Installation ![GitHub All Releases](https://img.shields.io/github/downloads/lptstr/painter/total.svg)
- Try using [Scoop](https://scoop.sh). <br>
  - ensure that the `extras` bucket is installed.
  - install:
    ```
    $ scoop install painter
    ```

- Or, if you don't want to use a package manager, just download `painter.exe` from the latest release in the latest [releases section](https://github.com/lptstr/painter/releases).

## Usage 
```
painter.exe [command] [args]
```

### Commands
- `get` <br>
  Retrieve the path to the current wallpaper. E.g.
  ```
  $ ./painter get
  "C:\\Users\\foo\\AppData\\Roaming\\Microsoft\\Windows\\Themes\\TranscodedWallpaper"
  ```
- `set [path]` <br >
  Set the wallpaper to an image. E.g.
  ```
  $ ./painter set "C:\Users\foo\Pictures\wallpaper.jpg"
  ```
## Inspiration
- [wallpaper-rs](https://github.com/MOZGIII/wallpaper-rs) by **@MOZGIII**
- [win-wallpaper](https://github.com/sindresorhus/win-wallpaper) by the venerable **@sindresorhus**
