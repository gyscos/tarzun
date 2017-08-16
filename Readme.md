Tarzun is a script to decompress a bunch of files and make a tar archive out of them.

It will uncompress all files given as arguments (currently zstd archives only) and output a tar archive of them as standard output.

The intended goal is to compress the output, making a single archive from a bunch of smaller files (and achieving better compression as a result).

Example usage:

```
$ tarzun small-*.tar.zst | zstd > big.tar.zst
```
