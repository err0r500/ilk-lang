# installation

```sh
npm run generate
```

then nvim conf
```lua
parser_config.ilk = {
    install_info = {
        url = "<path to ilk repo>/tree-sitter-ilk/",
        files = { "src/parser.c" },
    },
    filetype = { "ilk" },
}

vim.filetype.add({
    extension = {
        ilk = "ilk",
    },
})

vim.treesitter.language.register("ilk", "ilk")
vim.opt.runtimepath:append("<path to ilk repo>/tree-sitter-ilk")
```

then
```vim
:TSInstall ilk
```
