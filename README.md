# painter [![Build Status](https://travis-ci.org/lptstr/painter.svg?branch=master)](https://travis-ci.org/lptstr/painter)
Manage the Windows desktop wallpaper from the terminal

## Usage 
```
painter.exe [command] [args]
```
### Commands
- Get
  Retrieve the path to the current wallpaper. E.g.
  ```
  $ ./painter get
  "C:\\Users\\foo\\AppData\\Roaming\\Microsoft\\Windows\\Themes\\TranscodedWallpaper"
  ```
- Set \[path\]
  Set the wallpaper to an image. E.g.
  ```
  $ ./painter set "C:\Users\foo\Pictures\wallpaper.jpg"
  VERB 100 Attempting to set image to path C:\Users\foo\Pictures\wallpaper.jpg
  YAY! 102 Successfully changed wallpaper.
  ```
## Inspiration
- [wallpaper-rs](https://github.com/MOZGIII/wallpaper-rs) by **@MOZGIII**
- [win-wallpaper](https://github.com/sindresorhus/win-wallpaper) by the venerable **@sindresorhus**
