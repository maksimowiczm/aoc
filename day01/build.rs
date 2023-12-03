use cmake::Config;

fn main() {
    let dst = Config::new("CMakeLists.txt").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
}
