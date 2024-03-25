// #[cfg(feature = "logger")]
// fn link_loader_ffi() {
//     println!("cargo:rustc-link-lib=static=LoaderFFI");
// }

#[cfg(feature = "LuaEngine")]
fn link_luae() {
    println!("cargo:rustc-link-lib=static=LuaEngine");
}

fn main() {
    println!("cargo:rustc-link-search=lib");

    #[cfg(feature = "LuaEngine")]
    link_luae();
    // #[cfg(feature = "logger")]
    // link_loader_ffi()
}
