//Clear console
pub fn clear(){
    let ec:char = 27 as char;
    println!("{}c", ec);
}