// Rust Quine
fn main() {
    let a = "// Rust Quine\nfn main() {\n    let a = ";
    let b = "\n    print!(\"{}{:?};\n    let b = {:?};\n    {}\", a, a, b, b)\n}\n";
    
    print!("{}{:?};
    let b = {:?};
    {}", a, a, b, b)
}
