# BIDI Character Detector
This tool checks your files for existence of Unicode BIDI characters which can be misused for supply chain attacks to mitigate [CVE-2021-42574](https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-42574).
This tool was written in Rust and is distributes as an (< 3MB) docker compatible container to allow fast and easy usage.

For an explanation of the attack, have a look at [GitHub's blog entry](https://github.blog/changelog/2021-10-31-warning-about-bidirectional-unicode-text/) or the [original paper where the attack was published](https://trojansource.codes/).

## Installation
This package is mostly intended to be used via it's docker container. But local installation is possible of course.
Compilation requires at least Rust 1.56.0.
Clone this repository and run `cargo install --path .` to install the binary for your current user. After that, you can invoke the `bidi_detector` command.


## Usage
Running the tool via it's official docker container is probably the easiest way to get started.
To run it via docker, the following command should work to scan all files inside your current working directory:
```bash
docker run --rm -it -v $(pwd):/data ghcr.io/maweil/bidi_char_detector:latest
```

Depending on your system you may have to adapt this command slightly. If you use e.g. podman and have SELinux enabled, try the following command instead:

```bash
podman run --rm -it -v $(pwd):/data:Z ghcr.io/maweil/bidi_char_detector:latest
```

### Configuration
By default, all files will be checked. If you have binary files inside the current directory, the command will fail because it can't decode a non-UTF8 encoded file.
To adapt the command to your needs, place a file called `bidi_config.toml` inside the root of your project.
You can find an example for it in this repository, see an example below. The options will be described in more detail below the example: 

```toml
[general]
includes = [ 
    "src/**/*",
    "**/*.patch",
    "**/*.json",
    "**/Dockerfile",
    "test/*.js"
]
excludes = [
    ".git/*",
    "target/*"
]

[display]
show_details = true
```

#### General Settings
This section includes two arrays (`includes` and `excludes`) where you can specify patterns of files to be scanned (or to be excluded from the scan).
Please make sure your patterns actually match the files inside the directory, not the directory name itself, otherwise your files will not be scanned.
If you want to scan all files and only exclude e.g. your `.git` directory, the following configuration would do the trick:

```toml
[general]
includes = [ 
    "**/*"
]
excludes = [
    ".git/*",
]

[display]
show_details = true
```

If you want to intead explicitly define which folder contains your source files, the following configuration example would scan all files in the src directory (without ignoring anything):
```toml
[general]
includes = [ 
    "src/**/*"
]
excludes = [
]

[display]
show_details = true
```

#### Display Settings
If you enable the option `show_details`, the BIDI characters found in the respective files will be listed explicitly in addition to just the number of occurences found.

**Example:** `show_details = true`

```txt
src/lib.rs - 0 BIDI characters
src/main.rs - 0 BIDI characters
test/example-commenting-out.js - 6 BIDI characters
Found character RLO (Right-to-Left Override), test/example-commenting-out.js:4:3
Found character LRI (Left-to-Right Isolate), test/example-commenting-out.js:4:7
Found character PDI (Pop Directional Isolate), test/example-commenting-out.js:4:20
Found character LRI (Left-to-Right Isolate), test/example-commenting-out.js:4:22
Found character RLO (Right-to-Left Override), test/example-commenting-out.js:6:20
Found character LRI (Left-to-Right Isolate), test/example-commenting-out.js:6:24
Found 6 potentially dangerous Unicode BIDI characters!
```

**Example:** `show_details = false`

```txt
src/lib.rs - 0 BIDI characters
src/main.rs - 0 BIDI characters
test/example-commenting-out.js - 6 BIDI characters
Found 6 potentially dangerous Unicode BIDI characters!
```

## Credits
All credits for detecting the attack including the list of relevant BIDI characters go to the original authors of the corresponding paper. 
Please cite their original paper when building on their work.

The file `test/example-commenting-out.js` in this repository is a copy of [commenting-out.js](https://github.com/nickboucher/trojan-source/blob/main/JavaScript/commenting-out.js) in their original repository. It's licensing follows the [original repository](https://github.com/nickboucher/trojan-source) (MIT License)
It is used for test purposes only here.

```bibtex
@article{boucher_trojansource_2021,
    title = {Trojan {Source}: {Invisible} {Vulnerabilities}},
    author = {Nicholas Boucher and Ross Anderson},
    year = {2021},
    journal = {Preprint},
    eprint = {2111.00169},
    archivePrefix = {arXiv},
    primaryClass = {cs.CR},
    url = {https://arxiv.org/abs/2111.00169}
}
```
