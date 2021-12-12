# intel-backlight

This tool lets you change backlight brightness on laptops with intel_backlight.
I wrote it, because on my Lenovo E490 laptop `xbacklight` command fails with `No outputs have backlight property` message.

intel-backlight uses files in `/sys/class/backlight/intel_backlight` directory to read current brightness and change it to the desired value.

# How to install

You'll need [Rust](https://rustup.rs/) toolchain.

```
cargo build --release
```

The executable is located at `target/release/intel-backlight`. You can move or symlink it to a directory with executables, e.g. `/usr/local/bin` or `$HOME/.local/bin`, for convenience. Changing backlight brightness requires root privileges. You'll have to call it with `sudo` or give the executable `+s` flag, like this:

```
sudo chown root:root intel-backlight
sudo chmod +s intel-backlight
```

# How to use

Pass a number between 0 and 100 as the first argument to set backlight brightness to that many percent.

```
intel-backlight 10 # set backlight brightness to 10%
intel-backlight 80 # set backlight brightness to 80%
```

If you want to increase or decrease the current brightness, prefix the number with `-` or `+` symbol.

```
intel-backlight +10 # increase backlight brightness by 10% of the max value
                    # 0% becomes 10%, 50% becomes 60%, 100% stays at 100%
intel-backlight -10 # decrease backlight brightness by 10% of the max value
                    # 0% stays at 0%, 50% becomes 40%, 100% becomes 90%
```

Calling the tool with no arguments prints a help message.
