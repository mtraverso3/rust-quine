use sha256::{digest};

// Rust Hash Quine
fn main() {
    let a = "use sha256::{digest};\n\n// Rust Hash Quine\nfn main() {\n    let a = ";
    let b = "\n    print!(\"{}\", digest(format!(\"{}{:?};\n    let b = {:?};\n    {}\", a, a, b, b)))\n}\n";
    
    print!("{}", digest(format!("{}{:?};
    let b = {:?};
    {}", a, a, b, b)))
}
