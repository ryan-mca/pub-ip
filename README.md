# pub-ip
A quick rust script to print or write you public IPv4 address

## Commands
  --help    | -h  = print this help message

  --print   | -p4  = print your IPv4 address to stdout

  --printv6 | -p6 = print your IPv6 address to stdout

  --write   | -w4  = write your IPv4 address to pub-ipv4.txt

  --writev6 | -w6 = write your IPv6 address to pub-ipv6.txt

## Dependencies
### Debian:
```
sudo apt install libssl-dev cargo pkconfig gcc
```

### Archlinux:
```
sudo pacman -S openssl rust pkg-config gcc
```

### Gentoo
```
sudo emerge -av dev-libs/openssl dev-lang/rust sys-devel/gcc
```
## Compiling
```
git clone https://github.com/FIRED3STROYER/pub-ip
```
```
cd pub-ip
```
```
cargo build
```
### Optional
```
cargo install --path .
```
