# Proto Compilation

To generate the rust binding files for protobuf and gRPC the following command can be used:

```
REGEN_PROTOS=1 nix develop -c rust build
```