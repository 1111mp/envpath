# 🪟 envpath

📌 A tool to add or remove values from the Windows PATH variable. This project is inspired by [PathEd](https://github.com/awaescher/PathEd).

## Examples
###### Add a value to the Windows PATH:
`envpath.exe add "C:\Program Files\path"`

###### Remove a value from the Windows PATH:
`envpath.exe remove "C:\Program Files\path"`

###### Usage in a NSIS install script:
`envpath` is used in the [nvm-desktop](https://github.com/1111mp/nvm-desktop) NSIS install script to [add](https://github.com/1111mp/nvm-desktop/blob/a19ecd3c5d22d9766bf666ffd72b9c04e2ff4a56/src-tauri/templates/nsis-hooks.nsh#L9) and [remove](https://github.com/1111mp/nvm-desktop/blob/a19ecd3c5d22d9766bf666ffd72b9c04e2ff4a56/src-tauri/templates/nsis-hooks.nsh#L22) the application location to the Windows PATH.


## Noteworthy

`envpath` will ...
- ... just add values if they are new to the PATH. So it can be called multiple times.
- ... ignore removals if the value is not part of the PATH. So it can be called multiple times as well.