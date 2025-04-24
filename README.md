<p align="center">
  <img alt="" src="https://raw.githubusercontent.com/skippyr/river_dreams/refs/heads/master/assets/ornament.png" width="1020" />
</p>
<h1 align="center">≥v≥v&ensp;River Dreams&ensp;≥v≥v</h1>
<p align="center">
  <img alt="" src="https://img.shields.io/github/license/skippyr/river_dreams?style=plastic&label=%E2%89%A5%20license&labelColor=%2324130e&color=%23b8150d" />
  &nbsp;
  <img alt="" src="https://img.shields.io/github/v/tag/skippyr/river_dreams?style=plastic&label=%E2%89%A5%20tag&labelColor=%2324130e&color=%23b8150d" />
  &nbsp;
  <img alt="" src="https://img.shields.io/github/commit-activity/t/skippyr/river_dreams?style=plastic&label=%E2%89%A5%20commits&labelColor=%2324130e&color=%23b8150d" />
  &nbsp;
  <img alt="" src="https://img.shields.io/github/stars/skippyr/river_dreams?style=plastic&label=%E2%89%A5%20stars&labelColor=%2324130e&color=%23b8150d" />
</p>

## ❡ About
A tribal looking ZSH shell theme made to help you craft your most ambitious software projects. It is available for macOS and Linux.

<p align="center">
  <img alt="" src="https://raw.githubusercontent.com/skippyr/river_dreams/refs/heads/master/assets/preview.png" width="1020" />
</p>
<p align="center"><strong>Caption:</strong> a preview of the River Dreams theme with the <a href="https://github.com/skippyr/flamerial">Flamerial</a> palette.</p>

## ❡ Features
The left prompt shows:
- Your local IP address.
- Your disk usage and its status.
- Your battery charge and its status, if available.
- A calendar showing the weekday, month and day of month.
- A 24-hours clock showing the hours and minutes.
- A decorator when you are the root user.
- The exit code of the last command.
- The active Python virtual environment, if one has been sourced.
- The current directory path, abbreviated inside of Git repositories.
- The active Git branch, when inside of Git repositories.
- A decorator when you do not have permissions in the current directory.

The right prompt shows:
- A report of the total number of each entry type in the current directory:
  - Directories.
  - Files.
  - Sockets.
  - Fifos.
  - Block Devices.
  - Character Devices.
  - Symbolic Links *(they are not followed)*.
  - Hidden Entries.
  - Temporary Entries.
- The total number of jobs running in the background.

## ❡ Install
### Dependencies
The following dependencies must be installed before it:
- [**Rust Toolchain**](https://www.rust-lang.org): it will be used to build the project.
- **A font patched by the [Nerd Fonts project](https://www.nerdfonts.com/font-downloads):** it provides the pretty symbols used by the software.

Use your OS package manager or [HomeBrew](https://brew.sh) to install these packages.

It is also recommended to use a terminal with great Unicode support avoid installing multiple Nerd Fonts for the symbols used in the theme to appear correctly. The terminal recommended is [**Kitty**](https://github.com/kovidgoyal/kitty). You can download it by following the previous tip.

### Procedures
- Apply the Nerd Font installed in your terminal.
- Install the theme using `cargo`. In case of an error, check `cargo`'s output for troubleshooting.

```zsh
cargo install river_dreams;
```

- Initiate the theme in `~/.zshrc`:

```zsh
echo 'eval $(river_dreams init);' >> ~/.zshrc;
```

- Reopen the shell.

## ❡ Help
If you need help related to this project, open a new issue in its [issues pages](https://github.com/skippyr/river_dreams/issues) or send an [e-mail](mailto:skippyr.developer@icloud.com) describing what is going on.

## ❡ Contributing
This project is open to review and possibly accept contributions in the form of bug reports and suggestions. If you are interested, send your contribution to its [pull requests page](https://github.com/skippyr/river_dreams/pulls) or via [e-mail](mailto:skippyr.developer@icloud.com).

## ❡ License
This is free software licensed under the BSD-3-Clause License that comes WITH NO WARRANTY. Refer to the `LICENSE` file that comes in its source code for license and copyright details.

&ensp;
<p align="center"><sup>– 🐉❤️‍🔥 –</br><strong>≥v≥v&ensp;Here Be Dragons!&ensp;≥v≥</strong><br/>Made with love by 🍒 <a href="https://github.com/skippyr">skippyr</a></sup></p>
