# Heptgen

Easily generate C functions and prototypes from an heptagon interface file.

This program will generate an empty code file as well as two headers, one for function prototypes, one for output types.

## Example
A file names read.epi with the following content:
```
val fun myread(size:int) returns (samples:float^256)
```
Will generate the following files
- read_types.h
- read.h
- read.c

## Usage

```
$ hetpgen ./path/to/file.epi
```

## Available options

- `--help` show help informations
- `--force` bypass extension verification
- `--overwrite` force overwrite of existing file