// Generated by gir (https://github.com/gtk-rs/gir @ ef087c070d5b)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 1dc6c3826666)
// DO NOT EDIT

#[cfg(not(docsrs))]
use std::process;

#[cfg(docsrs)]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(docsrs))]
fn main() {
    if let Err(s) = system_deps::Config::new().probe() {
        println!("cargo:warning={s}");
        process::exit(1);
    }
}