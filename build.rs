fn main() {
    println!("cargo:rerun-if-changed=target-config/link.ld");
}
