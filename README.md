## Minebrew

>Warning! This is a work in progress, use at your own risk

1. [Planned Features](#planned-features)
1. [Installation](#installation)
1. [Usage](#usage)

Minebrew is a package manager for minecraft mods hosted on [modrinth](https://modrinth.com)

### Planned Features
- Installation
	- [x] Mod Installation
	- [ ] Mod Updates
	- [ ] Dependency Management
	- [ ] Incompatibility Management
	- [ ] Uninstalling
	- [ ] QOL features

- Searching
	- [ ] Basic searching functionality


- Configuration
	- [ ] Easy configuration editing

- General 
	- [ ] Cross platform releases


### Installation 
You will need an up-to-date version of rust installed. I personally prefer using rustup:

`$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Will install rustup and begin the installation process. Use the default options and when it is done clone the git repository wherever you would like:

`https://github.com/Brogramming-Inc/Minebrew.git`

next navigate into the `Minebrew` directory and and run the following command:

```
$ cd Minebrew
$ cargo install --path minebrew-bin
```

This will build Minebrew from source and place a binary in your `.cargo/bin` folder which on unix systems is usually

`$HOME/.cargo/bin/mbrew`

and on Windows 

`C:\Users\USERNAME\.cargo\bin\mbrew.exe`

From there you can do whatever you'd like with the binary.

**NOTEs**: 
- Minebrew assumes you have a .minecraft folder so please be sure to have Minecraft installed 
- the `reqwest` library requires libssl-dev to be installed, this is the case on most systems but if you get an error about openssl make sure to install this package

### Usage
```
USAGE:
    mbrew [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help               Print help information
    -m, --mc-dir <MC_DIR>    path to ".minecraft"
    -t, --target <TARGET>    override the default Minecraft version
    -V, --version            Print version information

SUBCOMMANDS:
    help       Print this message or the help of the given subcommand(s)
    install    Subcommand used to install mods
```

The only subcommand available at the moment is `install` but more are on the way. To download sodium and the fabric api run the following:

`mbrew install fabric-api sodium`

and Minebrew will find the mods and download them to your mods folder.

Should you have any issues please create one here on github and it will get fixed ASAP.

If you have any feature requests also submit an issue with the title "[Feature Request]" followed by the feature you have in mind with a description in the notes.
