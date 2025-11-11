# Change Log

All notable changes to the "Lenga" package.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/stpec/v2.0.0.html).

## [Unreleased]

## 1.0.2

### Added

- Tonic gRPC code generation added to source code to remove dependencies to a proto compiler for users.

## 1.0.1

### Fixed

- Bug where empty Return Statement, was incorrectly transpiled to plain text.

## 1.0.0

### Added

- Initial release of the Lenga package.
- Language library to manipulate Lenga C code.
- Bi-directional transpilation between Lenga C code to plain text C.
- Language like server to facilitate editors file manipulation.
- Merge algorithm to merge Lenga C code files.