
## **This** is a TUI for templated command execution

You can say it is similar to, but simpler than [navi](https://github.com/denisidoro/navi).

It aims to solve the "fuck i forgot the command to \_" or "this is so long to type every time" problems, just write it on a file.

## installation

It's just a binary, download and add to `$PATH`

Available systems:
- AArch64 (ARM64) Linux
```bash
curl -sSL https://github.com/cekita-ar/this/releases/download/experimental/aarch64-linux-this -o this && chmod +x ./this && sudo mv ./this ~/.local/bin
```

- x86_64 Linux
```bash
curl -sSL https://github.com/cekita-ar/this/releases/download/experimental/x86_64-linux-this -o this && chmod +x ./this && sudo mv ./this ~/.local/bin
```

- x86_64 Windows
```ps1
curl -sSL "https://github.com/cekita-ar/this/releases/download/experimental/x86_64-windows-this.exe" -OutFile "this.exe" ; Move-Item -Path "this.exe" -Destination "C:\Windows\System32\"
```

## Syntax and usage:

**This** searches for commands in many files, one of them is the global `$HOME/.local/this/global.this` file, and in the current directory for `*.this`.

Each file should have one command per line, and every line has two sections _(name and command)_ separated with `|`.

e.g.: `echo test | echo "Hello World!"`.

---

Now, there's also templates as I advertised.

On every command section you can use `{{foo}}` to create a template, it is required by default, but you can use `?` to indicate that it's optional. e.g.: `{{my_var?}}`.

You can also use lists with `{{foo:a,b,c}}` and `{{bar?:a,b,c}}`.

Keep in mind that there *are not* variables, these are simply a text that's shown on the UI. 

e.g.: `echo test | echo "{{text:a,b}}" && echo "{{text?}}"`

## Build from source

As usual, run `cargo build --release`

---

_in open-source we trust_
