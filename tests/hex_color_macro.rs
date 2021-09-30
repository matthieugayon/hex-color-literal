use hex::*;
use iced_native::Color;

#[test]
fn test_macro(){
    const white: Color = hex!("00ffff");
    println!("{:?}", white);
}