# MARS-T

**A reusable library and a CLI application to facilitate the colonisation of Mars**

[mars-t](https://github.com/piotrbajdek/mars-t) calculates the date and time on Mars based on an Earth's UTC, TAI or TT.

**1.** Mars Sol Date (MSD)

**2.** Martian Coordinated Time (MTC) / Airy Mean Time (AMT)

# USAGE

![help-image](https://github.com/piotrbajdek/mars-t/blob/main/docs/images/help-image.png?raw=true)

It is moreover possible to link against mars-t using its library as a dependency for other astronomical projects (see [public functions](https://docs.rs/mars-t/0.1.0/mars-t/#functions)).

# EXAMPLES

![example-image-1](https://github.com/piotrbajdek/mars-t/blob/main/docs/images/example-image-1.png?raw=true)

# INSTALLATION ON LINUX

[mars-t](https://github.com/piotrbajdek/mars-t) should run smoothly on **Windows** and **macOS**, and can be installed by the use of [cargo](https://www.rust-lang.org/tools/install). Yet, it is being developed and primarily tested on **Fedora Linux**.

mars-t v0.1.0:

– Was successfully tested on Fedora Linux 37, openSUSE Tumbleweed, and Ubuntu 22.10.

– Failed to run on Mageia 8 due to an old glibc version (required ≥2.34).

## METHOD 1 – BY THE USE OF CARGO

**[recommended for programmers]**

**1.** Install from crates.io by the use of cargo:

_cargo install mars-t_

By default, the file will be downloaded to `.cargo/bin/`, a hidden folder in your home directory.

**2a.** For convenience, you will probably want to copy mars-t to `/usr/bin/` as in Method 2 (3a, 3b).

**2b.** Alternatively, add `~/.cargo/bin` directory to your PATH variable (can be set up by [rustup](https://www.rust-lang.org/tools/install)).

## METHOD 2 – LINUX UNIVERSAL BINARIES

**1.** Download the distro-independent [binary](https://github.com/piotrbajdek/mars-t/releases/download/v0.1.0/mars-t) of mars-t from GitHub.

**2.** Make the file executable:

_sudo chmod +x ./mars-t_

**3a.** On most Linux distros, install mars-t via copying the binary to `/usr/bin/`:

_sudo cp mars-t /usr/bin/_

**3b.** On Fedora Silverblue / Kinoite:

_sudo cp mars-t /var/usrlocal/bin/_

## METHOD 3 – DISTRO-SPECIFIC PACKAGES

**[recommended for most users]**

Distro-specific packages are also available for download for [.rpm](https://github.com/piotrbajdek/mars-t/releases/download/v0.1.0/mars-t-0.1.0-1.x86_64.rpm)- and [.deb](https://github.com/piotrbajdek/mars-t/releases/download/v0.1.0/mars-t_0.1.0_amd64.deb)-based Linux distros. Installation instructions:

Fedora Linux / RHEL / openSUSE:

_sudo rpm -i mars-t-0.1.0-1.x86_64.rpm_

Fedora Silverblue / Kinoite:

_rpm-ostree install mars-t-0.1.0-1.x86_64.rpm_

Ubuntu:

_sudo dpkg -i mars-t_0.1.0_amd64.deb_

## METHOD 4 – MANUAL COMPILATION

Download and unpack the mars-t [source](https://github.com/piotrbajdek/mars-t/archive/refs/tags/v0.1.0.zip) from GitHub. Then, build and install the program:

_cargo build \--release && sudo cp target/release/mars-t /usr/bin/_
