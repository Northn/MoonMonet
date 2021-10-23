use monet_rs::theme::dynamic_color_scheme::DynamicColorScheme;
use monet_rs::theme::target_colors::TargetColors;
use monet_rs::colors::srgb::Srgb;
use libc::c_int;

//const CHROMA_MULTIPLIER: f64 = 1.0;

#[repr(C)]
struct SubColors {
    color_0: c_int,
    color_10: c_int,
    color_50: c_int,

    color_100: c_int,
    color_200: c_int,
    color_300: c_int,

    color_400: c_int,
    color_500: c_int,
    color_600: c_int,

    color_700: c_int,
    color_800: c_int,
    color_900: c_int,

    color_1000: c_int
}

#[repr(C)]
pub struct Colors {
    accent1: SubColors,
    accent2: SubColors,
    accent3: SubColors,

    neutral1: SubColors,
    neutral2: SubColors
}

#[no_mangle]
pub extern "C" fn moonmonet_build_colors(chroma_multiplier: f32, color: u32, accurate_shades: bool) -> Colors {
    let b: u8 = color as u8;
    let g: u8 = (color >> 8) as u8;
    let r: u8 = (color >> 16) as u8;
    
    let accents = DynamicColorScheme::new(TargetColors::new(chroma_multiplier as f64), Srgb::new(r, g, b), chroma_multiplier as f64, accurate_shades);
    Colors {
        accent1: SubColors {
            color_0: accents.get_accent1(0).quantize8() as c_int,
            color_10: accents.get_accent1(10).quantize8() as c_int,
            color_50: accents.get_accent1(50).quantize8() as c_int,

            color_100: accents.get_accent1(100).quantize8() as c_int,
            color_200: accents.get_accent1(200).quantize8() as c_int,
            color_300: accents.get_accent1(300).quantize8() as c_int,

            color_400: accents.get_accent1(400).quantize8() as c_int,
            color_500: accents.get_accent1(500).quantize8() as c_int,
            color_600: accents.get_accent1(600).quantize8() as c_int,

            color_700: accents.get_accent1(700).quantize8() as c_int,
            color_800: accents.get_accent1(800).quantize8() as c_int,
            color_900: accents.get_accent1(900).quantize8() as c_int,

            color_1000: accents.get_accent1(1000).quantize8() as c_int
        },
        accent2: SubColors {
            color_0: accents.get_accent2(0).quantize8() as c_int,
            color_10: accents.get_accent2(10).quantize8() as c_int,
            color_50: accents.get_accent2(50).quantize8() as c_int,

            color_100: accents.get_accent2(100).quantize8() as c_int,
            color_200: accents.get_accent2(200).quantize8() as c_int,
            color_300: accents.get_accent2(300).quantize8() as c_int,

            color_400: accents.get_accent2(400).quantize8() as c_int,
            color_500: accents.get_accent2(500).quantize8() as c_int,
            color_600: accents.get_accent2(600).quantize8() as c_int,

            color_700: accents.get_accent2(700).quantize8() as c_int,
            color_800: accents.get_accent2(800).quantize8() as c_int,
            color_900: accents.get_accent2(900).quantize8() as c_int,

            color_1000: accents.get_accent2(1000).quantize8() as c_int
        },
        accent3: SubColors {
            color_0: accents.get_accent3(0).quantize8() as c_int,
            color_10: accents.get_accent3(10).quantize8() as c_int,
            color_50: accents.get_accent3(50).quantize8() as c_int,

            color_100: accents.get_accent3(100).quantize8() as c_int,
            color_200: accents.get_accent3(200).quantize8() as c_int,
            color_300: accents.get_accent3(300).quantize8() as c_int,

            color_400: accents.get_accent3(400).quantize8() as c_int,
            color_500: accents.get_accent3(500).quantize8() as c_int,
            color_600: accents.get_accent3(600).quantize8() as c_int,

            color_700: accents.get_accent3(700).quantize8() as c_int,
            color_800: accents.get_accent3(800).quantize8() as c_int,
            color_900: accents.get_accent3(900).quantize8() as c_int,

            color_1000: accents.get_accent3(1000).quantize8() as c_int
        },
        neutral1: SubColors {
            color_0: accents.get_neutral1(0).quantize8() as c_int,
            color_10: accents.get_neutral1(10).quantize8() as c_int,
            color_50: accents.get_neutral1(50).quantize8() as c_int,

            color_100: accents.get_neutral1(100).quantize8() as c_int,
            color_200: accents.get_neutral1(200).quantize8() as c_int,
            color_300: accents.get_neutral1(300).quantize8() as c_int,

            color_400: accents.get_neutral1(400).quantize8() as c_int,
            color_500: accents.get_neutral1(500).quantize8() as c_int,
            color_600: accents.get_neutral1(600).quantize8() as c_int,

            color_700: accents.get_neutral1(700).quantize8() as c_int,
            color_800: accents.get_neutral1(800).quantize8() as c_int,
            color_900: accents.get_neutral1(900).quantize8() as c_int,

            color_1000: accents.get_neutral1(1000).quantize8() as c_int
        },
        neutral2: SubColors {
            color_0: accents.get_neutral2(0).quantize8() as c_int,
            color_10: accents.get_neutral2(10).quantize8() as c_int,
            color_50: accents.get_neutral2(50).quantize8() as c_int,

            color_100: accents.get_neutral2(100).quantize8() as c_int,
            color_200: accents.get_neutral2(200).quantize8() as c_int,
            color_300: accents.get_neutral2(300).quantize8() as c_int,

            color_400: accents.get_neutral2(400).quantize8() as c_int,
            color_500: accents.get_neutral2(500).quantize8() as c_int,
            color_600: accents.get_neutral2(600).quantize8() as c_int,

            color_700: accents.get_neutral2(700).quantize8() as c_int,
            color_800: accents.get_neutral2(800).quantize8() as c_int,
            color_900: accents.get_neutral2(900).quantize8() as c_int,

            color_1000: accents.get_neutral2(1000).quantize8() as c_int
        }
    }
}
