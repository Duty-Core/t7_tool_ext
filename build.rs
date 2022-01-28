fn main() {
    println!("cargo:rustc-link-arg=/ALIGN:128");
    println!("cargo:rustc-link-arg=/FILEALIGN:8");
    println!("cargo:rustc-link-arg=/EMITPOGOPHASEINFO");
    println!("cargo:rustc-link-arg=/DEBUG:NONE");

    println!("cargo:rustc-link-arg=/MERGE:.rdata=.text");
    println!("cargo:rustc-link-arg=/MERGE:.pdata=.text");
    
    println!("cargo:rustc-link-arg=/STUB:dos_stub.exe");

    // println!("cargo:rustc-link-arg=/ENTRY:DllMain");
    // println!("cargo:rustc-link-arg=/NODEFAULTLIB");
}
