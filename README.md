# hex-color-macros

Macros to generate `Color { r:f32, g:f32, b:f32, a:f32 }` structs based on an hexadecimal string literal.

**hex!** : Procedural macro for converting an hexadecimal color string into a color struct

This crate is a modified version of https://github.com/RustCrypto/utils/tree/master/hex-literal

Plays well with iced https://github.com/iced-rs
