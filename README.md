# D U M P
This is a quick and dirty CLI tool that:

- takes a folder,
- dumps it into a static website,
- which you can then throw up on a CDN or something.
    - (like GitHub Pages.)

Under the hood we use `ramhorns` for templating, and `rss` for, well, RSS.


## Usage
To dump a `website`:

```bash
# input is cwd
cd website
dump ../website-rendered # output path
```

> Note: Your dump doesn't have to be named `website`. You could name it "Carl's Magical Landfill" for all I care.

We only require one thing: a `.dump/` directory in `website`'s root.
The `.dump/` directory *must* have the following files:

| name | contents |
| --- | --- |
| `base.mustache` | Base html file into which all pages are rendered |
| `index.mustache` | Component for indexes |
| `text.mustache` | Component for plain text files |
| `binary.mustache` | Component for unknown binary files |

Defaults for these files can be copied from this repo's `./dump`.

You can also define additional components for *extensions*, say, Markdown.
These extensions must exist in `.dump/ext/`.
Dump provides content as markdown, if requested, so the following is possible:

```html
<!-- ./dump/ext/md.mustache -->
{{#md}}{{content}}{{/md}}
```

These extensions must take the form `<ext>.mustache`, where `<ext>` is the file extension, e.g. `file-name.md` has the `<ext>` of `md`.

## Templates
We use Mustache for templating (the `ramhorns` engine specifically). Here are the fields avaliable for templating:

### Base fields
Here are the base fields provided, that can be accessed directly, e.g. `{{content}}`:

- `m`: metadata about this current file
- `parent`: optional metadata about parent
- `children`: list of metadata about children, sorted by date
- `content`: Content as a string if utf-8 encoded
- `md`: markdown version of content

### Metadata fields
`m`, `parent`, and `children` are all metadata. Note that these are nested fields need to be accessed as `{{#field}}{{value}}{{/field}}`:

- `created`: time file was created, `YYYY-MM-DD`
- `modified`: time file was las modified, `YYYY-MM-DD`
- `title`: human-readable title of the file
- `link`: web link to this page, relative to root, i.e. `/this/is/some/page`
- `raw_link`: web link to this page's source, i.e. `/this/is/some/image.png`
- `slug`: Last part of the link, i.e. `page`
- `raw_slug`: Last part of link with extension, i.e. `image.png`

### Markdown
`md` is the markdown field. It has only one item:

- `content`: content, rendered as markdown

This field can be used in templates like this: `{{#md}}{{content}}{{/md}}`.

Nothing much else to add, I guess.

## Getting Creative
Extensions are pretty flexible. Say we have an image file (`.png`) and we want it to be rendered as a page. We can something like:

```html
<!-- ./dump/ext/png.mustache -->
<h1>{{#m}}{{title}}{{/m}}</h1>
<p>Taken on: {{#m}}{{created}}{{/m}}</p>
<img src="{{#m}}{{raw_link}}{{\m}}">
```

Here, the `img` tag links to the raw image.

I'm not going to write a full example here, but we could also use JS, to, say, write an interface to explore some data or something.

This is a very barebones project, and I have more planned. (Check out `IDEAS.md`).

I have this huge 2TB hard-drive with a collection of files I want to be able to toss up online and explore. My goal is to be able to `dump` the drive and host it as a professional-looking website.

## The `.dumpignore`
If you put a `.dumpignore` in `.dump`, files that match the `.dumpignore` will be ignored.
