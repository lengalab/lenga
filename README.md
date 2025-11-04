# Lenga

**A toolkit for working with the Lenga format.**

## Requirements

Lenga requieres the following tools to build:
- A Protobuf compiler

## Library

Lenga provides a library for manipulating language in Lenga format.

### Usage

```toml
[dependencies]
lenga = "1.0.0"
```

``` rust
use lenga::language::{Language, c::C};

fn main() {
    let lenga_file = File::open(&path).unwrap();
    let file_contents: Vec<u8> = lenga_file.bytes().map(|b| b.unwrap()).collect();
    let c_lenga_data = c.parse_nodes(file_contents).unwrap();

    fs::write(Path::new(&path), c_lenga_data).unwrap();
}
```

## Tools

Lenga provides some CLI tools to promote usage of Lenga format

### Transpile

Simple transpilation tool to convert to/from Lenga format. To use it simply run:

```
transpile <file>
```

#### Supported languages:

- C

### Lenga-Server

A language like server to facilitate editors file manipulation. Usage:

```
lenga-server
```

\* As of versión 1.0.0 the server always starts in port 49100.

### Merge

An intelligent merge algorithm for Lenga files. For CLI execution the following can be run:

```
merge <origin> <current> <other>
```

#### Git integration

Append to `.git/config`

```
[merge "c-lenga-driver"]
	name = A custom merge driver used to resolve conflicts in Lenga files
	driver = merge %O %A %B
```

Append to `.gitattributes`

``` 
*.c.lenga merge=c-lenga-driver"
```