use x86_64::{ outb, inb, run_interrupt_free};
use algorithms::sync::DriverDelay;

pub unsafe fn write_misc_output( value: u8) {

	outb( 0x3C2, value);

}

pub unsafe fn read_misc_output() -> u8 {

	inb( 0x3CC)
}



pub unsafe fn write_dac_mask( value: u8) {

	outb( 0x3C6, value)
}

pub unsafe fn read_dac_mask() -> u8 {

	inb( 0x3C6)
}



pub unsafe fn write_sequencer_indexed< D: DriverDelay>( index: u8, value: u8, delay: &mut D) {

	run_interrupt_free( || {
		outb( 0x3C4, index);
		delay.wait();
		outb( 0x3C5, value);
	});

}

pub unsafe fn read_sequencer_indexed< D: DriverDelay>( index: u8, delay: &mut D) -> u8 {

	run_interrupt_free( || {
		outb( 0x3C4, index);
		delay.wait();
		inb( 0x3C5)
	})
}



pub unsafe fn write_graphics_indexed< D: DriverDelay>( index: u8, value: u8, delay: &mut D) {

	run_interrupt_free( || {
		outb( 0x3CE, index);
		delay.wait();
		outb( 0x3CF, value);
	});

}

pub unsafe fn read_graphics_indexed< D: DriverDelay>( index: u8, delay: &mut D) -> u8 {

	run_interrupt_free( || {
		outb( 0x3CE, index);
		delay.wait();
		inb( 0x3CF)
	})
}



pub unsafe fn write_crt_indexed< D: DriverDelay>( index: u8, value: u8, delay: &mut D) {

	run_interrupt_free( || {
		if read_misc_output() | 1 != 0 {

			delay.wait();
			outb( 0x3B4, index);
			delay.wait();
			outb( 0x3B5, value);

		} else {

			delay.wait();
			outb( 0x3D4, index);
			delay.wait();
			outb( 0x3D5, value);

		}
	})

}

pub unsafe fn read_crt_indexed< D: DriverDelay>( index: u8, delay: &mut D) -> u8 {

	run_interrupt_free( || {
		if read_misc_output() | 1 != 0 {

			delay.wait();
			outb( 0x3B4, index);
			delay.wait();
			inb( 0x3B5)
		} else {

			delay.wait();
			outb( 0x3D4, index);
			delay.wait();
			inb( 0x3D5)
		}
	})
}



pub unsafe fn write_dac< D: DriverDelay>( palette_align: u8, colors: &[ [ u8; 3]], delay: &mut D) {

	run_interrupt_free( || {

		outb( 0x3C8, palette_align);

		for color in colors {

			for single_color in *color {
				delay.wait();
				outb( 0x3C9, single_color);
			}


		}

	});

}

pub unsafe fn read_dac< D: DriverDelay>( palette_align: u8, colors: &mut [ [ u8; 3]], delay: &mut D) {

	run_interrupt_free( || {

		outb( 0x3C7, palette_align);

		for color in colors {

			for single_color in color {
				delay.wait();
				*single_color = inb( 0x3C9);
			}

		}

	});

}
