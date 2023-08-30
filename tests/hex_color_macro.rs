use hex::*;
use iced_core::Color;

#[test]
fn test_macro(){
    const WHITE: Color = hex!("#000");
    println!("{:?}", WHITE);
}