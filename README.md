# GoodNotes Attachment Extractor (GAX)
Small rust command-line app to extract the `PDF` attachments from a `.goodnotes` file
(from the [GoodNotes App](https://www.goodnotes.com/)).

### Usage
```bash
gax <goodnotes-file> <output-dir>
```

### How this works
[GoodNotes](https://www.goodnotes.com/) files are basically zip-archives containing
the following directories (and files):
- `attachments`: contains attached/imported files in the `PDF` format
- `notes`: contains notes like text or strokes (in some weird GoodNotes format)
- `search`: contains some kind of search terms

The format of the `notes` folder is yet to be reverse-engineered and therefore, this project
settles for just the attachments for now, as they are easily extractable. If you know anything
about the more GoodNotes-specific formats, e.g. for converting the strokes to an output `PDF`,
feel free to open an issue or pull request.