use cc::Build;

fn main() {
    println!("cargo:rerun-if-changed=src/context_switch.S");
    Build::new()
        .file("src/asm_utils.s")
        .compile("asm");
}