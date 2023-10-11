# symfonysecret

<p align="left">
	<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/made%20with-Rust-red"></a>
	<a href="#"><img src="https://img.shields.io/badge/platform-osx%2Flinux%2Fwindows-blueviolet"></a>
</p>

`symfonysecret` is a proof of concept for exploring the Symfony framework using secret 🦀

- [Usage](#usage)
- [Compile](#compile)
- [Details](#details)


## Usage

- To use, just specify the following parameters:
```sh
symfonysecret -u <URL> --secret <SECRET> --command "id"
```

# Compile
```sh
cargo build --release
```

# Details

```
Symfony Fragment Secret Exploit

Usage: symfonysecret --url <URL> --secret <SECRET> --command <COMMAND>

Options:
  -u, --url <URL>          Insert URL
  -s, --secret <SECRET>    Insert secret
  -c, --command <COMMAND>  Insert command
  -h, --help               Print help
  -V, --version            Print version
```
