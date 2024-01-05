# findproject

findproject collects all your projects from locations specified in a simple YAML file.

## Installation

Install [fzf](https://github.com/junegunn/fzf) as a fuzzy finder.

Then clone the repository, build the project and move/link the files to a folder on your path.

```sh
git clone https://github.com/m-gail/findproject-rs
cargo build --release
# make sure ~/.local/bin is in your path
ln -s $(pwd)/fp_tmux ~/.local/bin/fp_tmux
ln -s $(pwd)/target/release/findproject-rs ~/.local/bin/findproject
```

## Usage

To use findproject with tmux, simple run `fp_tmux`.

The findproject executable can also be used on its own, see `findproject --help`.

## Configuration

```yaml
directories:
  - path: ~/Programming/ # directories in this will be printed
    sub_directories: # these directories will not be printed themselves
      - path: AppArmor # directories in ~/Programming/AppArmor this will be printed
  - path: ~/.config
    exclude: # these directories will also not be printed themselves
      - Nextcloud
      - GIMP
      - Thunar
      - xfce4
      - dconf
  - path: ~/Nextcloud/Notes
    only_self: true # ~/Nextcloud/Notes will be printed, but its contents won't
  - path: ~/.dotfiles
    only_self: true
  - path: ~/.ansible
    only_self: true
```
