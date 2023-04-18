fn main() {
    println!("cargo:rerun-if-changed=target-config/link.ld");
    println!("cargo:rerun-if-changed=target-config/nanos_layout.ld");
    println!("cargo:rerun-if-changed=target-config/nanosplus_layout.ld");
    println!("cargo:rerun-if-changed=target-config/nanox_layout.ld");
}
