# A quick overview of project structure

Here you can find the source. The process of dumping is pretty simple:

- get CLI arguments
- extract templates and configs from `.dump`
- walk the current directory:
    - build a list of subpages
    - render subpages recursively
    - build index of directory
- build rss feed

The name of each file is pretty self-explanatory.
