# hex-color-macros

Macros to generate `Color { r:f32, g:f32, b:f32, a:f32 }` structs based on an hexadecimal string literal.

**hex!** : Procedural macro for converting an hexadecimal color string into a color struct

**palette_dark_101!** : Procedural macro for generating a fixed array of 101 darkening shades of a given hex color (index 0), black at index 101

**palette_light_101!** : Procedural macro for generating a fixed array of 101 lightening shades of a given hex color (index 0), white at index 101


This crate is a modified version of https://github.com/RustCrypto/utils/tree/master/hex-literal

Plays well with iced https://github.com/iced-rs 
If the Iced Color struct definition is in scope (https://github.com/iced-rs/iced/blob/master/core/src/color.rs),
you don't have to define the Color struct !!
