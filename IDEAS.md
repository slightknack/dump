# `slightknack.dev` version 2

# Goals
- Static, deployable on cdn
- Versioned w/ Git
- Hostable on Github/Pages
- Takes a folder and dumps a website
- Can embed articles
- Can embed small projects
- Generated pages are static and optimized
- Logical mapping from files to locations
- Standalone Rust binary

# Ideas
- Files prefixed with `_` are accessible but hidden.
- Use git history to determine created / edited.
- Collate subfolders into feeds.
    - RSS feeds auto-generated
- Files are named `this is the title.extension`
    - slug becomes `this-is-the-title`.
    - two files in the same folder is not allowed.
    - `index.something` is the root
    - folders without index generate an index
- HTML files do not have styling as default, act as local templates
    - mustache, passed object with environment context.
- Markdown files are rendered out into posts
    - could this be done via template?
- Other files are rendered out in a file-discovery mode.
- a route with an extension returns the raw content

```
+ website
|-+ _static
| |- style.css
| |- base.html
|-+ blog
| |- My First Post.md
| |- My Second Post.md
|- index.md
```

Rendering technique:
```
start with the root folder
(1) enter the folder
if index.* in folder
    go to (2) with index.*
pages = for each item in folder
    if item is a folder go to (1)
    if not go to (2) with item
    if item is hidden, don't hide
make feed from subpages
render index with feed if markdown
render raw index if html

(2) load the file
use git to find last time modified
if file is markdown, render and insert into default html template
if file is html, do the raw html
otherwise, insert into default html template
emit rendered page without extension
emit raw page with extension
```

# More ideas

- `.dump` folder is used for configuration
- `.something` files are ignored
- `.dump/ext/<ext>.mustache` defines a template for `<ext>` file type
- `.dump/base.mustache` defines the base template

# Even more ideas

- html, markdown, and templates need to be included by default.
    - markdown especially, to render server-side
    - we can actually do this automatically!
- files without extension shouldn't have trailing dot.
- more clearly define boundaries
    - this project is like a shell script but verbose
- handle errors more gracefully

# Better rendering technique:
```
copy the directory to a new location
walk the new directory:
    for each file, render out copy as html
    for each directory, render out all sub-files
```

# More ideas

- Builds a search index
- Can write JS to use search index.
- Trim content to get short description in Metadata
