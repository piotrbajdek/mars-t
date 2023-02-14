# MARS-T

![mars-t-logo](https://github.com/piotrbajdek/mars-t/blob/main/docs/images/mars-t-logo.png?raw=true)

**A comprehensive library and command-line interface (CLI) tool designed to aid in the colonisation of Mars, by providing reusable resources and functionalities**

[mars-t](https://github.com/piotrbajdek/mars-t) determines the date and time on Mars by utilising Earth's Coordinated Universal Time (UTC), International Atomic Time (TAI), or Terrestrial Time (TT).

It provides two types of time:

**1.** Mars Sol Date (MSD)

**2.** Martian Coordinated Time (MTC)/ Airy Mean Time (AMT)

# USAGE

![help-image](https://github.com/piotrbajdek/mars-t/blob/main/docs/images/help-image.png?raw=true)

It is moreover possible to link against mars-t using its library as a dependency for other astronomical projects (see [public functions](https://docs.rs/mars-t/0.2.1/mars_t/all.html#functions)).

# EXAMPLES

![example-image-1](https://github.com/piotrbajdek/mars-t/blob/main/docs/images/example-image-1.png?raw=true)

# INSTALLATION ON LINUX

The current version of mars-t (v0.2.1) has been verified to work properly on Fedora Linux 37 and Ubuntu 22.10.

## METHOD 1 – USING CARGO

**[Recommended for programmers]**

**1.** To install mars-t from [crates.io](https://crates.io/crates/mars-t), use the following [cargo](https://www.rust-lang.org/tools/install) command:

_cargo install mars-t_

The executable will be saved in the hidden `.cargo/bin/` directory within your home directory.

**2a.** For easy access, you may want to copy the mars-t file to either the `/usr/bin/` or the `~/.local/bin/` directory. This can be done by following the instructions in Method 3 (3a, 3b, 3c).

**2b.** As an alternative, you can add the `~/.cargo/bin/` directory to your system's PATH variable, which can be configured using [rustup](https://www.rust-lang.org/tools/install).

## METHOD 2 – SNAP INSTALLATION

The software can be obtained through the [Snap Store](https://snapcraft.io/mars-t) and installed with just one command:

_sudo snap install mars-t_

## METHOD 3 – UNIVERSAL LINUX BINARIES

**1.** To install mars-t, first download the distro-independent [binary](https://github.com/piotrbajdek/mars-t/releases/download/v0.2.1/mars-t) from GitHub.

**2.** Then, make the file executable by running the command:

_sudo chmod +x ./mars-t_

**3a.** On most Linux distributions, install mars-t by copying the binary to `/usr/bin/`:

_sudo cp mars-t /usr/bin/_

**3b.** For Fedora Silverblue / Kinoite, use this command:

_sudo cp mars-t /var/usrlocal/bin/_

**3c.** As an alternative, consider installing locally in the `~/.local/bin/` directory:

_cp mars-t $HOME/.local/bin/_

## METHOD 4 – DISTRO-SPECIFIC PACKAGES

**[Recommended for most users]**

Distro-specific packages for [.rpm](https://github.com/piotrbajdek/mars-t/releases/download/v0.2.1/mars-t-0.2.1-1.x86_64.rpm) and [.deb](https://github.com/piotrbajdek/mars-t/releases/download/v0.2.1/mars-t_0.2.1_amd64.deb)-based Linux distributions are also available for download. To install mars-t on different Linux distributions, follow these instructions:

Fedora Linux / RHEL / openSUSE:

_sudo rpm -i mars-t-0.2.1-1.x86_64.rpm_

Fedora Silverblue / Kinoite:

_rpm-ostree install mars-t-0.2.1-1.x86_64.rpm_

Ubuntu:

_sudo dpkg -i mars-t_0.2.1_amd64.deb_

## METHOD 5 – MANUAL COMPILATION

First, download and unpack the mars-t [source code](https://github.com/piotrbajdek/mars-t/archive/refs/tags/v0.2.1.zip) from GitHub. Next, to build and install the program, use the command:

_cargo build \--release && sudo cp target/release/mars-t /usr/bin/_
