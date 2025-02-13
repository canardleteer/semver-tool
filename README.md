# semver-tool

A simple tool for working with [Semantic Versioning](https://semver.org/)
on the command line.

Semantic Versioning seems simple, but in many cases, it's not implemented
correctly, and people only consider the MAJOR.MINOR.PATCH cases. When this
happens, you may find yourself needing to do surgery on lists of versions
in a pipeline or even just to reason around someones releases.

The Rust ecosystem, as well as most Cloud Native patterns have adopted
SemVer. Not everyone has, and sometimes a little grease is needed to get
systems back on track where there's divergence.

It can be damaging to a project, when semantic versioning is only
partially or incorrectly implemented. It's a foundational communication
mechanism for engineers, and should be treated with the care and diligence
it's owed.

This is a single tool to provide support for that purpse.

## Opinions

Where appropriate opinions on the spec have been made, they have been
listed in the CLI documentation.

## Installing

Until I put this on [crates.io](https://crates.io), you can install this
via:

```shell
cargo install --path .
```

## Output

Currently, the following output types are: `yaml`, `text`, `json`.

I favor the `yaml` output, and have made that the default.

## Caveat

It's been awhile since I've done much Rust programming, but trying to
make something useful while getting back into it. I'm sure I'm making all
the normal mistakes I made the first time I starting writing Rust all
over again.

## Todo

[ ] Need status code responses options
[ ] Possibly remove "text" output, or just make it really nice.
[ ] Additional language filter implementations
[ ] Generate random semantic version lists for helping build tests
[ ] Property testing
[ ] Github Actions + release-plz
[ ] Tests around CLI usage, not just internal libs.

## filter-test

The `filter-test` subcommand will allow you to test a filter on a version.

```shell
# Passing test
$ semver-tool filter-test ">=1.0.3" 1.0.3
---
pass: true

# Failing test
$ semver-tool filter-test ">=1.0.3" 1.0.1
---
pass: false
```

## explain

The `explain` subcommand will break down a version by components.

The `semver-tool explain --help` command has some useful
information regarding "why" the output may appear "over-stringified"
in the breakdown.

```shell
$ semver-tool explain 10.1.4-a.b.c+sda.4
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

## compare

```shell
# simple case
$ semver-tool compare 1.2.3 2.2.2
---
semantic_ordering: Less
lexical_ordering: Less

# comparing 2 "equal" versions
$ semver-tool compare 2.2.2+abc 2.2.2
---
semantic_ordering: Equal
lexical_ordering: Greater
```

## sort

The `sort` command is somewhat complex, but offers 2 differet modes of input:

- CLI arguments
- reading from stdin

It is recommended that you read `semver-tool sort --help`, but here are some
examples. If you're wondering why you may sometimes get different results
than these, it's once again, helpful to read the `--help`.

### CLI arguments

```shell
# simple cli argument sorting
$ semver-tool sort 1.2.3 3.2.1 2.2.2
---
versions:
  1.2.3:
  - 1.2.3
  2.2.2:
  - 2.2.2
  3.2.1:
  - 3.2.1

# simple cli argument sorting, reverse ordering
$ semver-tool sort -r 1.2.3 3.2.1 2.2.2
---
versions:
  3.2.1:
  - 3.2.1
  2.2.2:
  - 2.2.2
  1.2.3:
  - 1.2.3

# filtering
$ semver-tool sort -f ">=2" -r 1.2.3 3.2.1 2.2.2
---
versions:
  3.2.1:
  - 3.2.1
  2.2.2:
  - 2.2.2
```

### stdin

```shell
# stdin argument sorting
$ cat example-data/short-good-versions.txt | semver-tool sort
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
$ cat example-data/short-good-versions.txt | semver-tool sort -r
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
$ cat example-data/short-good-versions.txt | semver-tool sort -r -f '*'
---
versions:
  0.2.0:
  - 0.2.0
  0.0.2:
  - 0.0.2
  0.0.1:
  - 0.0.1

# flattening (not recommended)
$ cat example-data/short-good-versions.txt | semver-tool sort --flatten
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
$ cat example-data/short-good-versions.txt | semver-tool  -o text sort --flatten -r -f "*" 
0.2.0
0.0.2
0.0.1
```
