# Nested Dir File Copy

## Summary
nested_dir_file_copy is a rust program that recursivly searches an input directory and copies specified file types to an output directory. File structure is flattened.
This program is primitive and is my first attempt at creating a rust program.

## Usage

``nested_dir_file_copy.exe [OPTIONS] <SOURCE> <DESTINATION>``

```

ARGS:
    <SOURCE>
            Path to the root source directory

    <DESTINATION>
            Path to the output directory

OPTIONS:
    -d, --dry
            Do not copy any files output to terminal instead

    -e, --exts <EXTS>
            Comma Seperated file extentions that will be copied

            [default: jpg,png,gif,mp4,mov,webp,pdf,jpeg]

    -h, --help
            Print help information

    -n, --nothumb
            Place all thumbnails in main output directory instead of nested Thumbs directory

    -V, --version
            Print version information
```