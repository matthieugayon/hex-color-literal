use hex::*;
use iced_native::Color;

#[test]
fn test_macro(){
    const WHITE: Color = hex!("#000");
    println!("{:?}", WHITE);
}