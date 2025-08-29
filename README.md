<p align="center">
  <img alt="A tribal ornament made out of a triangle pattern" src="https://raw.githubusercontent.com/skippyr/river_dreams/refs/heads/master/assets/ornament.png" width="1020" />
</p>
<h1 align="center">≥v≥v&ensp;River Dreams&ensp;≥v≥v</h1>
<p align="center">
  <img alt="A badge containing the license the repository is under" src="https://img.shields.io/github/license/skippyr/river_dreams?style=plastic&label=%E2%89%A5%20license&labelColor=%2324130e&color=%23b8150d" />
  &nbsp;
  <img alt="A badge containing the latest tag from the repository" src="https://img.shields.io/github/v/tag/skippyr/river_dreams?style=plastic&label=%E2%89%A5%20tag&labelColor=%2324130e&color=%23b8150d" />
  &nbsp;
  <img alt="A badge containing the total number of commits in the repository" src="https://img.shields.io/github/commit-activity/t/skippyr/river_dreams?style=plastic&label=%E2%89%A5%20commits&labelColor=%2324130e&color=%23b8150d" />
  &nbsp;
  <img alt="A badge containing the total number of stars in the repository" src="https://img.shields.io/github/stars/skippyr/river_dreams?style=plastic&label=%E2%89%A5%20stars&labelColor=%2324130e&color=%23b8150d" />
</p>
<p align="center">
  <img alt="A badge containing the macOS logo" src="https://img.shields.io/badge/macOS-000000?logo=apple&logoColor=F0F0F0" />
  <img alt="A badge containing the Linux logo" src="https://img.shields.io/badge/Linux-FCC624?logo=linux&logoColor=black" />
</p>
<p align="center"><sup>Since February 2023</sup></p>

## ❡ About
A tribal looking ZSH shell theme made to help you craft your most ambitious software projects. It is available for macOS and Linux.

<p align="center">
  <img alt="" src="https://raw.githubusercontent.com/skippyr/river_dreams/refs/heads/master/assets/preview.png" width="1020" />
</p>
<p align="center"><strong>Caption:</strong> a preview of the River Dreams theme applied running <a href="https://github.com/eza-community/eza">eza</a> in <a href="https://github.com/kovidgoyal/kitty">Kitty</a> on macOS. The palette used is <a href="https://github.com/skippyr/flamerial">Flamerial</a> and font is <a href="https://www.monolisa.dev">Monolisa</a> (with fallback to <a href="https://github.com/ryanoasis/nerd-fonts">Symbols Nerd Font</a>). Background is AI art and provided as an wallpaper port by the <a href="https://github.com/skippyr/flamerial">Flamerial</a> palette.</p>

## ❡ Features
The left prompt shows:

<p>
  <details>
    <summary>Click to expand (...)</summary>
    <ul>
      <li>Your local IP address.</li>
      <li>Your disk usage and its status.</li>
      <li>Your battery charge and its status, if available.</li>
      <li>A calendar showing the weekday, month and day of month.</li>
      <li>A 24-hours clock showing the hours and minutes.</li>
      <li>A decorator when you are the root user.</li>
      <li>The exit code of the last command.</li>
      <li>The active Python virtual environment, if one has been sourced.</li>
      <li>The current directory path, abbreviated inside of Git repositories.</li>
      <li>The active Git branch and a decorator when it is dirty, when inside of Git repositories.</li>
      <li>A decorator when you do not own the current directory.</li>
    </ul>
  </details>
</p>

The right prompt shows:

<p>
  <details>
    <summary>Click to expand (...)</summary>
    <ul>
      <li>
        The total number of each entry type in the current directory:
        <ul>
          <li>Files.</li>
          <li>Directories.</li>
          <li>Sockets.</li>
          <li>Fifos.</li>
          <li>Block devices.</li>
          <li>Character devices.</li>
          <li>Symlinks <em>(they are not followed)</em>.</li>
          <li>Hidden entries.</li>
          <li>Temporary entries.</li>
        </ul>
      </li>
      <li>The total number of jobs running in the background.</li>
    </ul>
  </details>
</p>

## ❡ Install
### Dependencies
The following dependencies must be installed before it:
- ZSH: this is the shell the theme runs on. If you are on macOS, this is already the default one. On Linux, only a few distributions come with it by default, if that is not your case, you would need to change it manually.
- [**Rust Toolchain**](https://www.rust-lang.org): it will be used to build the project from source.
- **A font patched by the [Nerd Fonts project](https://www.nerdfonts.com/font-downloads):** it provides the pretty symbols used by the software. Alternatively, you can use the font containing just its symbols as a fallback to an unpatched one if your terminal supports. Avoid having multiple ones installed due to possible font conflicts.
- **A terminal with good unicode support:** it will be used to run the shell. It is highly recommended to use [**Kitty**](https://github.com/kovidgoyal/kitty) due to its amazing features and traits required to render the fonts used in the theme.

Use your OS package manager or [HomeBrew](https://brew.sh) to install these packages.

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

## ❡ Special Thanks
Huge thanks to [unixorn](https://github.com/unixorn) who has seen potential in this software and considered it an awesome plugin. If you want to see other amazing creations made by other developers, not just including themes, remember to check out its [Awesome ZSH Plugins repository](https://github.com/unixorn/awesome-zsh-plugins).

## ❡ License
This is free software licensed under the BSD-3-Clause License that comes WITH NO WARRANTY. Refer to the `LICENSE` file that comes in its source code for license and copyright details.

&ensp;
<p align="center"><sup>– 🐉❤️‍🔥 –</br><strong>≥v≥v&ensp;Here Be Dragons!&ensp;≥v≥</strong><br/>Made with love by 🍒 <a href="https://github.com/skippyr">skippyr</a></sup></p>
