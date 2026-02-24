fn main() {
    let name = "binary";
    let enc = encoding::label::encoding_from_whatwg_label(name);
    println!("{:?}", enc.map(|e| e.name()));
    
    let name = "unknown";
    let enc = encoding::label::encoding_from_whatwg_label(name);
    println!("{:?}", enc.map(|e| e.name()));
}
