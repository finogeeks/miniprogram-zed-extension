# Miniprogram zed extension

<p align="center">
  <a href="./README_en.md">English</a> |
  <a href="./README.md">简体中文</a>
</p>

[![](https://img.shields.io/badge/Powered%20by-FinClip-lightgrey)](https://finclip.com)

zed 微信小程序开发插件

## 安装

1. 克隆项目到本地
2. 打开Zed编辑器，从右上角的菜单中选择`Extensions`，然后点击`Install Dev Extension`按钮，选择项目目录即可

## 安装小程序预览工具

1. 下载小程序预览工具  
  [Windows](https://www-cdn.finclip.com/desktop-sdk/preview-tool/miniprogram-builder-win.exe)  
  [Mac](https://www-cdn.finclip.com/desktop-sdk/preview-tool/miniprogram-builder-macos)  
2. 配置 Zed 编辑器的 Tasks

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

## 使用

用 Zed 编辑器打开小程序项目，使用快捷键`Cmd+Shift+P`打开命令面板，输入`tasks` Miniprogram Preview 即可预览小程序，此时编辑小程序代码并保存会自动重启预览窗口，可以实时看到小程序的改动

https://github.com/finogeeks/miniprogram-zed-extension/assets/1532625/a3f9c7e4-2093-4c90-beee-89dd03237921
