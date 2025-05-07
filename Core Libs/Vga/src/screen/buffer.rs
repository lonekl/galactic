use super::{ /*Char, Color16x16,*/ Char16Color};

const COLOR_TEXT_BUFFER: * mut [ Char16Color; 0x4000 /* Data type has 2 bytes so array size must be 2 times smaller. */] = 0xb8000 as * mut [ Char16Color; 0x4000];
//const MONOCHROME_TEXT_BUFFER: * mut [Char; 0x8000] = 0xb0000 as * mut [Char; 0x8000];
//const GRAPHIC_COLOR16_BUFFER: * mut [Color16x16; 0x10000] = 0xa0000 as * mut [Color16x16; 0x10000];
//const GRAPHIC_COLOR256_BUFFER: * mut [u8; 0x10000] = 0xa0000 as * mut [u8; 0x10000];

#[ cfg( target_arch = "x86_64")]
pub unsafe fn write_raw_text_color_16x16( position: usize, char_color: Char16Color) {

	( *COLOR_TEXT_BUFFER)[ position] = char_color;

}

#[ cfg( target_arch = "x86_64")]
pub unsafe fn read_raw_text_color_16x16( position: usize) -> Char16Color {

	( *COLOR_TEXT_BUFFER)[ position]
}



#[ cfg( target_arch = "x86_64")]
pub unsafe fn write_xy_text_color_16x16( x: usize, y: usize, char_color: Char16Color) {

	write_raw_text_color_16x16( x + y * 80, char_color)
}

#[ cfg( target_arch = "x86_64")]
pub unsafe fn read_xy_text_color_16x16( x: usize, y: usize) -> Char16Color {

	read_raw_text_color_16x16( x + y * 80)
}

