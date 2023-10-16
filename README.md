# halifuda-rust-theme
 This is a Rust theme for VSCode. 
 The dark theme is forked from [Dracula](https://github.com/dracula/visual-studio-code).
 The light theme is forked from [Solarized](https://github.com/ryanolsonx/vscode-solarized-theme).
 
 Install this package to your extensions dir like `~/.vscode/extensions/`.

 Requires rust-analyzer for full experience.

## Main features

- A standalone color for lifetime.
- Differentiate trait and struct.
- Sync colors of function args and macro args.

## Install

Install from the release VSIX. You can use "install from VSIX" in VSCode extension manager.

## Build

To build, you need to first install Node.js. Then install `vsce`:
```
npm install -g @vscode/vsce
```

Then, under this dir, use:
```
vsce package
```

Then you can find the VSIX file.

See [oficial-website](https://code.visualstudio.com/api/working-with-extensions/publishing-extension) for more information.