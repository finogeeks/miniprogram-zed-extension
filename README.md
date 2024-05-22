# FinClip zed extension

一个用于FinClip小程序的Zed插件

## 安装

1. 克隆项目到本地
2. 打开Zed编辑器，从右上角的菜单中选择`Extensions`，然后点击`Install Dev Extension`按钮，选择项目目录即可

## 安装小程序预览工具

1. 下载FinClip小程序预览工具  
  [Windows](https://www-cdn.finclip.com/desktop-sdk/preview-tool/offline-applet-builder-win.exe)  
  [Mac](https://www-cdn.finclip.com/desktop-sdk/preview-tool/offline-applet-builder-macos)  
2. 配置Zed编辑器的Tasks

```json
[
  {
    "label": "FinClip Preview",
    "command": "/path/to/file/offline-applet-builder-win.exe",
    "args": [],
    "env": {},
    "use_new_terminal": false,
    "allow_concurrent_runs": false,
    "reveal": "always"
  }
]
```

## 使用

用Zed编辑器打开小程序项目，使用快捷键`Cmd+Shift+P`打开命令面板，输入`tasks`选择FinClip Preview即可预览小程序，此时编辑小程序代码会自动重启预览窗口