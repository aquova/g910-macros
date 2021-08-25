# G910 Macros

A userspace driver for adding G-key macro support for Logitech's G910 Orion keyboard.

While other similar programs do exist, those implementations would hog the keyboard interface, breaking RGB support. This program attempts to alleviate that problem. Support for different profiles via the M-keys is not supported, and they are simply rebound to other keys.

I will say that while this program is functional, it is quite jank, and probably doesn't follow Linux best practices in the slightest. Use at your own risk.

By default, this program binds G1-G9 to F13-F21, M1-M3 to F22-F24, and MR to a Calculator button (I ran out of F-keys). You can then use your system's keyboard shortcut tool to bind these to other programs.

## Installation

- With the Rust compiler installed, run `make` and `make install` to install the program
- Run `systemctl start g910-macros.service` to start program, and `systemctl enable g910-macros.service` to start at boot.

## Uninstallation

- Run `systemctl stop g910-macros.service` to stop the driver, then run `make uninstall` to remove the programs from the system.

## TODO

- Add support for a configuration file.

## References

If you wish to change what each key is bound to, a list of possible uinput keycodes can be found here:

- Standard keyboard keys - https://docs.rs/uinput/0.1.3/uinput/event/keyboard/enum.Key.html
- Misc. functions - https://docs.rs/uinput/0.1.3/uinput/event/keyboard/enum.Misc.html
