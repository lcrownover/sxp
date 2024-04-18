# Components to build

- two positional arguments

`sxp 'notation' 'expression'

For example:

From: `sxp 'n01-n03' '{}.talapas.uoregon.edu.yml'`
To: `n01.talapas.uoregon.edu.yml,n02.talapas.uoregon.edu.yml,n03.talapas.uoregon.edu.yml`

- parsing into groups
    - each comma outside of a range bracket should be a "group"
    - within a group:
        - each character outside of the bracket will be repeated for each expanded string
        - hyphens are ranges, iterate through integers, convert to multiple groups
        - commas are separators between groups

done:
    - expansion of expression

todo:
    - fix newline separator to actually be newlines
    - hostname expansion
