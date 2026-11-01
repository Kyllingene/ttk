package = "ttk"
version = "1.1.0-1"
source = {
   url = "git+https://github.com/Kyllingene/ttk.git"
}
dependencies = {
   "lua >= 5.1",
   "luarocks-build-rust-mlua"
}
build = {
   type = "rust-mlua",
   modules = {
      ["ttk"] = "ttk"
   }
}
