fn main(){
    let name = "jayesh";
    match name {
        "jayesh"=>println!("name is jayesh"),
        "rust"=>println!("name is rust"),
        "cargo"=>println!("name is cargo"),
        "vscode"=>println!("name is vscode"),
        "dsa"=>println!("name is dsa"),
        _=>println!("no match found"),
    }
}