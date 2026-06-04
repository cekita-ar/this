
## **This** is a TUI for templated command execution

You can say it is similar to, but simpler than [navi](https://github.com/denisidoro/navi).

It aims to solve the "fuck i forgot the command to \_" or "this is so long to type everytime" problems, just write it on a file.

## installation

It's just a bash file, download and add to `$PATH`

Tho it depends on:
- [fzf](https://github.com/junegunn/fzf)
- bash
- standar POSIX utils _(awk, sed, tr, cat, touch)_

For most systems you should be just good to go with:

```
curl -sSL https://raw.githubusercontent.com/cekita-ar/this/refs/heads/main/this -o this && chmod +x ./this && sudo mv ./this /usr/local/bin
```

## Syntax and usage:

**This** searches for commands in many files, one of them is the global `$HOME/.this` file, and in the local directory `*.this`

Each file should have one command per line, and every line has two sections _(name and command)_ separated with `|`

e.g.: `echo test | echo "Hello World!"`

---

Now, theres also templating as I advertised

On every command section you can use `{{foo}}` to declare an string variable, by default it is required, but you can also use `?` to indicate that it's optional. e.g.: `{{my var?}}`

> Note that names with spaces are supported even tho it is recommendedn't

You can also use lists with `{{foo:a,b,c}}` and `{{bar?:a,b,c}}`

If you redeclare a variable it would simply be used **as is**, since there's no variable shadowing or anything like that

e.g.: `echo test | echo "{{text:a,b}}" && echo "{{text?}}"` <- here the `{{text?}}` would be interpreted as `{{text}}`

---

_in open-source we trust_