# Merge Driver

Let's you resolve merge conflicts in lenga files.

## Usage

```
merge origin current other
```

## Git integration

Append to `.git/config`

```
[merge "c-lenga-driver"]
	name = A custom merge driver used to resolve conflicts in lenga files
	driver = merge-driver %O %A %B
```

Append to `.gitattributes`

``` 
*.c.lenga merge=c-lenga-driver"
```