# jav-parser

## Usage
`jav-parser [SOURCE] [DESTINATION] [INTERVAL] [MINIMUM SIZE IN BYTES]`

## Use case
Let's say that there is a directory containing some files, calling it `source`:

```
source/
├── website.com@ABC-100/
│   └── hd800.com@ABC-100.mp4
├── website.com@ABC-101/
│   └── extra-information.txt
│   └── advertisement-video.mp4
│   └── website.com@ABC-101.mp4
├── website.com@ABC-102/
│   └── extra-information.txt
│   └── random-folder-name/
│       └── website.com@ABC-102.mp4
```

By running a command:
`$ jav-parser source/ destination/ 60 400000000`

This will:
1. Read the `source` directory every 60 seconds.
2. Create a directory (if not exists) and creates a folder with parsed jav names.
3. Symlinks files above 400000000 bytes into the destination directory.

Which would look like:
```
destination/
│   └── ABC-100/
│      └── ABC-100.mp4
│   └── ABC-101/
│      └── ABC-101.mp4
│   └── ABC-102/
│      └── ABC-102.mp4
```

Much cleaner and prevents junk in the destination.

## Recommended setup
By using metatube-server pointed at the destination directory, clean metatata for JAV can be added into jellyfin. Files are downloaded into the source directory, meaning that no data is changed from the source whatsoever.
