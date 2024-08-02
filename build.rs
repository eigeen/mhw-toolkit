#[cfg(feature = "lua_engine")]
fn link_luae() {
    println!("cargo:rustc-link-lib=static=LuaEngine");
}

fn main() {
    println!("cargo:rustc-link-search=lib");

    #[cfg(feature = "lua_engine")]
    link_luae();
}
