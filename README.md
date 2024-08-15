## Scantron

Organize all of your files under a directory and its subdirectories into a JSON object with a key of each unique file type

Oh – and it's insanely fast

<sub>⚠️ This project is in a usable state but still under construction</sub>

#### Usage

Display output to terminal for current directory

```bash
scantron
```

Pass in a folder

```bash
scantron -p <PATH_TO_FOLDER>
```

Run in quiet mode and generate a `scantron.json` file in your current directory

```bash
scantron -d
```

#### Potential Use Cases

* You can use the generated `scantron.json` file in your CI/CD pipeline for custom DAST scans or other vulnerability checks
* Keep an organized, running file of all file types that exist in your codebase
* Complex file management tracking purposes

> Inspired by [this repo](https://github.com/saarthdeshpande/github-repo-parser)
