# River Dreams
## About
A tribal looking ZSH shell theme to help you craft your most ambitious software projects. It is available for macOS and Linux.

![](https://gitlab.com/skippyr/river_dreams/-/raw/master/Assets/Preview.png)

## Install
### Prerequisites
In order to use this software, the following requirements must be fulfilled:
- Have the latest version of the [Rust Toolchain](https://www.rust-lang.org) installed.
- Have a font patched by the [Nerd Fonts project](https://www.nerdfonts.com/font-downloads) installed and set in the terminal.
- Use a terminal with great unicode support and rendering, as [Kitty](https://github.com/kovidgoyal/kitty), for example.

### Procedures
- Apply the Nerd Font installed in your terminal.
- Install the theme using `cargo`. In case of an error, check `cargo`'s output for troubleshooting.

```zsh
cargo install river_dreams
```

- Initiate the theme in `~/.zshrc`. If you are using OhMyZSH, manually remove or comment out the line that defines the `ZSH_THEME` variable:

```zsh
echo 'eval $(river_dreams init);' >> ~/.zshrc
```

- Reopen the shell.

## Help
If you need help related to this project, open a new issue in its [issues pages](https://gitlab.com/skippyr/river_dreams/issues) or send an [e-mail](mailto:skippyr.developer@icloud.com) describing what is going on.

## Contributing
This project is open to review and possibly accept contributions in the form of bug reports and suggestions. If you are interested, send your contribution to its [merge requests](https://gitlab.com/skippyr/river_dreams/merge_requests) page or via [e-mail](mailto:skippyr.developer@icloud.com).

## Copyright
This is free software licensed under the BSD 3-Clause License that comes WITH NO WARRANTY. Refer to the `LICENSE` file that comes in its source code for license and copyright details.
