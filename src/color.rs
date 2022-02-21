pub type Color = [u8; 4];

fn mix_channels(c1: u8, c2: u8, a1: u8, a2: u8) -> u8 {
    ((c1 as i32 * (255 - a1) as i32 + c2 as i32 * a2 as i32) / 255) as u8
}

pub(crate) fn mix_colors(c1: Color, c2: Color) -> Color {
    let a = 255 - ((255 - c1[3]) as i32 * (255 - c2[3] / 255) as i32);
    let r = mix_channels(c1[0], c2[0], c1[3], c2[3]);
    let g = mix_channels(c1[1], c2[1], c1[3], c2[3]);
    let b = mix_channels(c1[2], c2[2], c1[3], c2[3]);

    [r, g, b, a as u8]
}
