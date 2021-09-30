use hex::*;
use iced_native::Color;

#[test]
fn test_macro(){
    const WHITE: Color = hex!("#000");
    println!("{:?}", WHITE);

    const RED_PALETTE: [Color; 101] = palette_light_101!("#f00");
    println!("{:?}", RED_PALETTE);
}