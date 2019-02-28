# painter [![Build Status](https://travis-ci.org/lptstr/painter.svg?branch=master)](https://travis-ci.org/lptstr/painter)
Manage the Windows desktop wallpaper from the terminal

## Installation
Try using [Scoop](https://scoop.sh). <br>
- First, add the LPTSTR-Scoop bucket (if you havn't already):
  ```
  $ scoop bucket add lptstr https://github.com/lptstr/lptstr-scoop
  ```
- Now install the application:
  ```
  $ scoop install painter
  ```
- Then enjoy :smile:
  ```
  $ painter get
  ```

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
  VERB 100 Attempting to set image to path C:\Users\foo\Pictures\wallpaper.jpg
  YAY! 102 Successfully changed wallpaper.
  ```
## Inspiration
- [wallpaper-rs](https://github.com/MOZGIII/wallpaper-rs) by **@MOZGIII**
- [win-wallpaper](https://github.com/sindresorhus/win-wallpaper) by the venerable **@sindresorhus**
