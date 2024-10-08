# rtail

A `tail` clone written in Rust.

"tail" is a command line utility which gives you the last n lines (default 10) of STDIN or a text file. You can also "watch", or follow, the contents continually as it comes in.

# Usage Examples

```
# Handle text from Stdin
cat /path/to/file | rtail -n 20

# Handle text in a file
rtail -n 1 /etc/passwd
```

```
Usage: rtail [OPTIONS] [FILE]

If FILE is not present, it will read from STDIN.

Arguments:
  [FILE]  The file to read from (optional)

Options:
  -n, --number <NUMBER>  The number of lines to print [default: 10]
  -h, --help             Print help
  -V, --version          Print version
```

# Limitations

Input data stream or input file MUST contain UTF-8 otherwise it will be rejected.