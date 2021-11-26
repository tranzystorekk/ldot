# ldot

## About

List hidden files and directories.

This is a paper-thin wrapper around `ls` + the `glob` crate.
It includes `ls` flags for basic formatting and sorting, like `-lh`, `-tr`, etc.

In fact, after resolving the dotfile glob, `ldot` calls `exec` to transform into `ls`.

## Supported platforms

Currently only works on Unix family platforms.
