# sem-tool

> **A simple tool for working with [Semantic Versioning](https://semver.org/) on the command line.**

[![Crates.io](https://img.shields.io/crates/v/sem-tool?style=flat-square)](https://crates.io/crates/sem-tool)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](LICENSE-APACHE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/canardleteer/sem-tool/testing.yml?branch=main&style=flat-square)](https://github.com/canardleteer/sem-tool/actions/workflows/testing.yml?query=branch%3Amain)

Semantic Versioning seems simple, but in many cases, it's not implemented
correctly, and people only consider the MAJOR.MINOR.PATCH cases. When this
happens, you may find yourself needing to do surgery on lists of versions
in a pipeline or even just to reason around someones releases.

The Rust ecosystem, as well as most Cloud Native patterns have adopted
SemVer. Not everyone has, and sometimes a little grease is needed to get
systems back on track where there's divergence.

It can be damaging to a project, when Semantic Versioning is only
partially or incorrectly implemented. It's a foundational communication
mechanism for engineers, and should be treated with the care and diligence
it's owed.

This is a single tool to provide support for that purpse.

## Opinions

Where appropriate opinions on the spec have been made, they have been
listed in the CLI documentation.

## Known Limitations

- We (currently) use the [semver crate](https://crates.io/crates/semver), which
  has some limitations not present in the spec.
  - `u64::MAX` is the maximum a value can be in any of `MAJOR`, `MINOR` or `PATCH`.
  - The maximum number of comparators for a filter, is 32.

## Installing

- The easy way, if you use Rust, is:

```shell
cargo install sem-tool
```

- [Releases](https://github.com/canardleteer/sem-tool/releases) have various installer
  patterns for multiple Operating Systems and Package Management tools.

## Running

Your best place to start, is:

```shell
sem-tool --help
```

## Output

Currently, the following output types are: `yaml`, `text`, `json`.

I favor the `yaml` output, and have made that the default.

Exit Status is available either by default in obvious cases, or by flag in less obvious cases.

## Subcommands

### `filter-test`

The `filter-test` subcommand will allow you to test a filter on a version.

```shell
# Passing test
$ sem-tool filter-test ">=1.0.3" 1.0.3
---
pass: true
$ echo $?
0

# Failing test
$ sem-tool filter-test ">=1.0.3" 1.0.1
---
pass: false
$ echo $?
1
```

### `validate`

The `validate` subcommand just helps a script determine if a string is a valid Semantic Version or not.

```shell
# Passing test
$ sem-tool validate 1.0.3-x+m
---
valid: true
$ echo $?
0

# Failing test
$ sem-tool validate a.b.c
---
valid: false
$ echo $?
1
```

### `explain`

The `explain` subcommand will break down a version by components.

The `sem-tool explain --help` command has some useful
information regarding "why" the output may appear "over-stringified"
in the breakdown.

```shell
$ sem-tool explain 10.1.4-a.b.c+sda.4
---
major: 10
minor: 1
patch: 4
prerelease_string: a.b.c
prerelease:
- kind: Ascii
  value: a
- kind: Ascii
  value: b
- kind: Ascii
  value: c
build_metadata_string: sda.4
build-metadata:
- kind: Ascii
  value: sda
- kind: Numeric
  value: '4'
```

### `compare`

```shell
# simple case
$ sem-tool compare 1.2.3 2.2.2
---
semantic_ordering: Less
lexical_ordering: Less
$ echo $?
0

# simple case with status code reporting enabled
$ sem-tool compare -e 1.2.3 2.2.2
---
semantic_ordering: Less
lexical_ordering: Less
$ echo $?
100

# comparing 2 "equal" versions
$ sem-tool compare 2.2.2+abc 2.2.2
---
semantic_ordering: Equal
lexical_ordering: Greater
$ echo $?
0

# comparing 2 "equal" versions with status code reporting enabled
$ sem-tool compare -e 2.2.2+abc 2.2.2
---
semantic_ordering: Equal
lexical_ordering: Greater
$ echo $?
112

$ sem-tool compare -es 2.2.2+abc 2.2.2
---
semantic_ordering: Equal
lexical_ordering: Greater
$ echo $?
0
```

### `sort`

The `sort` command is somewhat complex, but offers 2 differet modes of input:

- CLI arguments
- reading from standard input

It is recommended that you read `sem-tool sort --help`, but here are some
examples. If you're wondering why you may sometimes get different results
than these, it's once again, helpful to read the `--help`.

#### `sort` with CLI arguments

```shell
# simple cli argument sorting
$ sem-tool sort 1.2.3 3.2.1 2.2.2
---
versions:
  1.2.3:
  - 1.2.3
  2.2.2:
  - 2.2.2
  3.2.1:
  - 3.2.1

# simple cli argument sorting, reverse ordering
$ sem-tool sort -r 1.2.3 3.2.1 2.2.2
---
versions:
  3.2.1:
  - 3.2.1
  2.2.2:
  - 2.2.2
  1.2.3:
  - 1.2.3

# filtering
$ sem-tool sort -f ">=2" -r 1.2.3 3.2.1 2.2.2
---
versions:
  3.2.1:
  - 3.2.1
  2.2.2:
  - 2.2.2
```

#### `sort` with standard input

```shell
# stdin argument sorting
$ cat example-data/short-good-versions.txt | sem-tool sort
---
versions:
  0.0.0-alpha.0:
  - 0.0.0-alpha.0+metadata
  0.0.1:
  - 0.0.1
  0.0.2:
  - 0.0.2
  0.2.0:
  - 0.2.0
  1.0.0-rc-2:
  - 1.0.0-rc-2+aaaaaa
  1.0.0-rc-2.0:
  - 1.0.0-rc-2.0+aaa.0
  - 1.0.0-rc-2.0+dddddd
  99.99.0-rc1.0:
  - 99.99.0-rc1.0

# reverse ordering
$ cat example-data/short-good-versions.txt | sem-tool sort -r
---
versions:
  99.99.0-rc1.0:
  - 99.99.0-rc1.0
  1.0.0-rc-2.0:
  - 1.0.0-rc-2.0+aaa.0
  - 1.0.0-rc-2.0+dddddd
  1.0.0-rc-2:
  - 1.0.0-rc-2+aaaaaa
  0.2.0:
  - 0.2.0
  0.0.2:
  - 0.0.2
  0.0.1:
  - 0.0.1
  0.0.0-alpha.0:
  - 0.0.0-alpha.0+metadata

# filtering (see --help regarding how this filter applies)
$ cat example-data/short-good-versions.txt | sem-tool sort -r -f '*'
---
versions:
  0.2.0:
  - 0.2.0
  0.0.2:
  - 0.0.2
  0.0.1:
  - 0.0.1

# flattening (not recommended)
$ cat example-data/short-good-versions.txt | sem-tool sort --flatten
---
versions:
- 0.0.0-alpha.0+metadata
- 0.0.1
- 0.0.2
- 0.2.0
- 1.0.0-rc-2+aaaaaa
- 1.0.0-rc-2.0+aaa.0
- 1.0.0-rc-2.0+dddddd
- 99.99.0-rc1.0

# flat list of latest matching a filter as a plain list
$ cat example-data/short-good-versions.txt | sem-tool  -o text sort --flatten -r -f "*" 
0.2.0
0.0.2
0.0.1
```

## Todo

- [X] Need status code responses options
- [ ] Possibly remove "text" output, or just make it really nice.
- [ ] Additional language filter implementations
- [ ] Generate random semantic version lists for helping build tests
- [X] Github Actions + release-plz
- [ ] Commands that take stdin, should probably take file inputs too.
- [ ] Property testing
  - [X] There is work on [this branch](https://github.com/canardleteer/sem-tool/tree/proptest), but it needs a reorg.
- [ ] CLI Testing (probably) with `assert_cmd`
  - [X] `compare`
  - [X] `filter-test`
  - [X] `sort`
  - [X] `explain`
  - [ ] all subcommands
  - make these far more robust
  - restructure or add a framework to make them more legible & composable
  - validate all output types
