# Miniprogram zed extension

<p align="center">
  <a href="./README_en.md">English</a> |
  <a href="./README.md">简体中文</a>
</p>

[![](https://img.shields.io/badge/Powered%20by-FinClip-lightgrey)](https://finclip.com/en)

Zed miniprogram development plugin

## Installation

1. Clone the project to local
2. Open the Zed editor, select `Extensions` from the menu in the upper right corner, then click the `Install Dev Extension` button, and select the project directory

## Install the mini program preview tool

1. Download the mini program preview tool  
  [Windows](https://www-cdn.finclip.com/desktop-sdk/preview-tool/miniprogram-builder-win.exe)  
  [Mac](https://www-cdn.finclip.com/desktop-sdk/preview-tool/miniprogram-builder-macos)  
2. Configure the Tasks of the Zed editor

```json
[
  {
    "label": "Miniprogram Preview",
    "command": "/path/to/file/miniprogram-builder-win.exe",
    "args": [],
    "env": {},
    "use_new_terminal": false,
    "allow_concurrent_runs": false,
    "reveal": "always"
  }
]
```

## Usage

Open the mini program project with the Zed editor, use the shortcut `Cmd+Shift+P` to open the command panel, enter `tasks` Miniprogram Preview to preview the mini program, at this time editing and saving the mini program code will automatically restart the preview window, you can see the changes of the mini program in real time

https://github.com/finogeeks/miniprogram-zed-extension/assets/1532625/a3f9c7e4-2093-4c90-beee-89dd03237921

## RoadMap

- [ ] Upload code
- [x] Preview miniprogram
- [ ] Build npm package
