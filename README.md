# def
def is a small tool that keeps track of file and directory descriptions. It
works by mapping a path to a description, storing it in a config file, and
retrieving it when needed.

## Install
```
cargo install def
```

## Usage
```
def keeps track of file and directory descriptions for you.

Usage

  def <path>                        Print description of file/dir at path.
  def add <path> <description>      Add a description for file/dir at path.
  def pattern <path> <description>  Add a pattern to describe children of dir. A
                                    wildcard in the pattern will be replaced with
                                    the child's name.

Descriptions

  To describe a file or directory use add sub-command which simply maps a description
  to an absolute path.

  The pattern sub-command is used to describe all children of a directory using a
  common trait. When pattern is used, a description is mapped to a dir, but is used
  only to describe its children. If a wildcard "*" exists in the pattern, it will
  be replaced by the child's name.

  For example:

  $ def pattern dir "* is a child of dir"
  $ def dir/temp
  /path/to/dir/temp: temp is a child of dir

Descriptions and patterns are kept in ~/.config/def/config.json which maps each
description to an absolute path and can be added to or adjusted manually.
```

### Example
```console
$ def add path/directory "This is a directory."
$ def pattern path/directory "* is a child of path/directory."

$ def path/directory
/absolute/path/directory: This is a directory.

$ cd path/directory
$ def temp.txt  # wildcard is replaced by child's name (temp.txt).
/absolute/path/directory/temp.txt: temp.txt is a child of path/directory
```
