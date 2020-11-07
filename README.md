# ddir
ddir is a small tool that keeps track of directory descriptions. It works by
mapping a path to a description, storing it in a config file, and retrieving
it when needed.

## Usage
```
ddir keeps track of directory descriptions for you.

Usage

  ddir <path>                         Print description of dir at path.
  ddir -add <path> <description>      Add a description for dir at path.
  ddir -pattern <path> <description>  Add a pattern to describe children of dir. A
                                      wildcard in the pattern will be replaced with
                                      the child directory's name.

Descriptions

  To describe a directory use -add flag which simply maps a description to a path.

  The -pattern flag is used to describe all children of a directory using a common
  trait. When -pattern is used, a description is mapped to a dir, but is used only
  to describe its children. If a wildcard " * " exists in the pattern, it will be
  replaced by the child's name.

  For example:

    $ ddir -pattern "/dir" "* is a child of /dir"
    $ ddir "/dir/temp"
    /dir/temp: temp is a child of /dir

Descriptions and patterns are kept in ~/.config/ddir/config.json and can be added to
or adjusted manually.
```

### Example

```console
$ ddir -add /path/directory "This is a directory."
$ ddir -pattern /path/directory "* is a child of /path/directory."
$ ddir /path/directory
/path/directory: This is a directory.
$ ddir /path/directory/temp  # wildcard is replaced by child's name (temp).
/path/directory/temp: temp is a child of /path/directory
```
