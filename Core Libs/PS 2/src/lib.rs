//! PS/2 keyboard driver, TODO complete this driver as it'sn't completed.

use x86_64::{ outb, inb};
use algorithms::buffers::;

/// Error code first.
const ERR0: u8 = 0;
/// Error code second.
const ERR1: u8 = 0xff;
/// Self test passed.
const SPASS: u8 = 0xaa;
/// Echo response.
const ECHO: u8 = 0xee;
/// Command acknowledged.
const ACK: u8 = 0xfa;
/// Self test failed code first.
const SFAIL0: u8 = 0xfc;
/// Self test failed code second.
const SFAIL1: u8 = 0xfd;
/// Resend the commmand or data.
const RESEND: u8 = 0xfe;
/// Used in keymap static to make writing of it easier.
const Z: ( u8, char) = ( 0, 0 as char);

static mut INPUT_RETRY: bool = false;
/// Do keyboard currently resets.
static mut RESET: bool = false;
/// Queue of commands to be sent.
static mut COMMANDS: Buffer< Command, QUEUE_LEN> = Buffer::new( Command{ comm: 0, data: 0, state: 0, resend: 0 });
/// Current selected keymap.
static mut CURRENT_KEY_MAP: usize = 1;
/// Reached key map layer.
static mut CURRENT_KEY_MAP_LAYER: usize = 0;
/// Setted key mapping (default to `US QWERTY`).
/// Characters have u8 type too for defining do it is scan code for pressing or releasing, (maybe in future it will have more uses).
static KEY_MAP: /* Mappings. */ &'static [ /* Mapping layers. */ &'static [ ( &'static [ u8], &'static [ ( u8, char)])]] = &[

	&[],
	&[

		( &[ 0xe0, 0xe1], &[
			Z, ( 1, 27 as char),
			( 1, '1'), ( 1, '2'), ( 1, '3'), ( 1, '4'), ( 1, '5'), ( 1, '6'), ( 1, '7'), ( 1, '8'), ( 1, '9'), ( 1, '0'), ( 1, '-'), ( 1, '='),
			( 1, 8 as char) /* Backspace. */, ( 1, '\t'),
			( 1, 'q'), ( 1, 'w'), ( 1, 'e'), ( 1, 'r'), ( 1, 't'), ( 1, 'y'), ( 1, 'u'), ( 1, 'i'), ( 1, 'o'), ( 1, 'p'), ( 1, '['), ( 1, ']'),
			( 1, '\n'), ( 1, 17 as char),
			( 1, 'a'), ( 1, 's'), ( 1, 'd'), ( 1, 'f'), ( 1, 'g'), ( 1, 'h'), ( 1, 'j'), ( 1, 'k'), ( 1, 'l'), ( 1, ';'), ( 1, '\''), ( 1, '`'),
			( 1, 16 as char), ( 1, '\\'), ( 1, 'z'), ( 1, 'x'), ( 1, 'c'), ( 1, 'v'), ( 1, 'b'), ( 1, 'n'), ( 1, 'm'), ( 1, ','), ( 1, '.'), ( 1, '/'),
			( 1, 2 as char) /* Right shift. */, ( 1, '*'), ( 1, 18 as char) /* Left alt. */, ( 1, ' '), ( 1, 20 as char) /* Caps lock. */,
			( 1, 112 as char), ( 1, 113 as char), ( 1, 114 as char), ( 1, 115 as char), ( 1, 116 as char), ( 1, 117 as char), ( 1, 118 as char), ( 1, 119 as char), ( 1, 120 as char), ( 1, 121 as char) /* F1-10. */, ( 1, 144 as char) /* Number lock. */, ( 1, 145 as char) /* Scroll lock. */,
			( 1, 103 as char), ( 1, 104 as char), ( 1, 105 as char) /* Number pad 7-9. */, ( 1, 109 as char) /* Number pad - */, ( 1, 100 as char), ( 1, 101 as char), ( 1, 102 as char) /* Number pad 4-6. */, ( 1, 107 as char) /* Number pad + */, ( 1, 97 as char), ( 1, 98 as char), ( 1, 99 as char) /* Number pad 1-3. */, ( 1, 96 as char) /* Number pad 0. */, ( 1, 190 as char) /* Number pad decimal. */,
			Z, Z, Z, Z, ( 1, 122 as char), ( 1, 123 as char) /* F11-12. */,
		]),
		( &[ 0x1d, 0x2a, 0xb7], &[
			Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, ( 1, 177 as char) /* Previous track. */,
			Z, Z, Z, Z, Z, Z, Z, Z, ( 1, 176 as char) /* Next track. */,
			Z, Z, ( 1, '\r') /* Number pad enter. */, ( 1, 1 as char) /* Control. */, Z, Z, Z, ( 1, 173 as char) /* Mute. */, Z /* Calculator - not supported. */, ( 1, 179 as char) /* Play. */, Z, ( 1, 178 as char) /* Stop - pressed. */,
			Z, Z, Z, Z, Z, Z, Z, Z, Z, ( 1, 174 as char) /* Volume down. */, Z, ( 1, 175 as char) /* Volume up. */, Z, Z /* WWW home - not supported. */, Z, Z, ( 1, 111 as char) /* Number pad divide. */, Z, Z, ( 1, 225 as char) /* Alt gr (right alt). */,
			Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, ( 1, 36 as char) /* Home. */, Z /* Cursor up - not supported. */, ( 1, 33 as char) /* Page up. */, Z, Z /* Cursor left - not supported. */, Z, Z /* Cursor right - not supported. */, Z, ( 1, 35 as char) /* End. */, Z /* Cursor down - not supported. */, ( 1, 34 as char) /* Page down. */, ( 1, 45 as char) /* Insert */, ( 1, 46 as char) /* Delete. */,
			Z, Z, Z, Z, Z, Z, Z, Z /* Left gui - not supported. */, Z /* Right gui - not supported. */, Z /* Apps - not supported. */, Z /* Power - not supported. */, ( 1, 95 as char) /* Sleep. */, Z, Z, Z, Z /* Wake - not supported. */, Z, Z, Z /* WWW search - not supported. */, Z /* WWW favorites - not supported. */, ( 1, 168 as char) /* WWW refresh. */, Z /* WWW stop - not supported. */, Z /* WWW forward - not supported. */, Z /* WWW back - not supported. */, Z /* My computer - not supported. */, Z /* Email - not supported. */, Z /* Media select - not supported. */, 
			Z, Z,
		]),
		( &[ 0xe0, 0x45], &[]),
		( &[ 0xe1], &[
			Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, ( 1, 44 as char) /* Print screen - pressed. */,
			Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, Z, ( 0, 44 as char) /* Print screen - released. */
		]),

	],

];

#[ derive( Clone, Copy)]
struct Command {

	comm: u8,
	data: u8,
	state: u8, // 0 bit: command was send, 1 bit: do data should be send.
	resend: u8,

}

/// Called at keyboard interrupt.
pub fn interrupt() {

	unsafe {

		if can_read() {

			read();

		} else {

			INPUT_RETRY = true;

		}

	}

}

fn read() {

	unsafe {
		let input = inb( 0x60);

		match input {

			ERR0 | ERR1 => {

				// TODO

			},
			ECHO => {

				// TODO

			},
			ACK => {
				let comm = COMMANDS.head();

				if comm.state & 0b1 != 0 || comm.state & 0b10 == 0 { // If command was send. || If data should not be send.

					COMMANDS.reject();

				} else {

					comm.state |= 0b1;

				}

			},
			RESEND => {
				let comm = COMMANDS.head();

				comm.resend += 1;

				if comm.resend == 3 { // command not supported or hardware failure

					COMMANDS.reject();

				}

			},
			_ => {
				let mut get_scan = true;
				let layer = KEY_MAP[ CURRENT_KEY_MAP][ CURRENT_KEY_MAP_LAYER];

				if RESET {

					get_scan = false;
					RESET = false;

					if input == SPASS {



					} else if input == SFAIL0 || input == SFAIL1 {



					}

				} else {

					for next in layer.0 {

						if input == *next {

							CURRENT_KEY_MAP_LAYER += 1;
							get_scan = false;

						}

					}

				}

				if get_scan && layer.1.len() > if CURRENT_KEY_MAP == 1 { input & 0b01111111 } else { input } as usize {
					let mut press = true;
					let addr = if CURRENT_KEY_MAP == 1 && layer.1.len() <= input as usize {

						if input & 0b10000000 != 0 {

							press = false;

						}

						input & 0b01111111
					} else {

						input
					};
					let ( attr, key) = layer.1[ addr as usize];

					if CURRENT_KEY_MAP != 1 {

						press = attr & 0b1 != 0;

					}

					CURRENT_KEY_MAP_LAYER = 0;

					if key as u32 != 0 {

						// Here the received key should be interpreted (key and press and key variables say about it).

					}

				}

			},

		}

	}

}

/// called at timer interrupt
pub fn timer() {

	unsafe {

		if INPUT_RETRY && can_read() {

			read();
			INPUT_RETRY = false;

		} else {

			if COMMANDS.len() != 0 && can_write() {
				let comm = COMMANDS.head();

				if comm.state & 0b1 == 0 { // Command was not send.

					outb( 0x60, comm.comm);

				} else if comm.state & 0b10 != 0 { // Command was send. && If data should be send.

					outb( 0x60, comm.data);

				}

			}

		}

	}

}

pub fn reinit() {

	reset();

}

pub fn reset() {

	command_send( 0xff, 0, false);

}

pub fn enable_scanning() {

	command_send( 0xf4, 0, false);

}

pub fn disable_scanning() {

	command_send( 0xf5, 0, false);

}

pub fn set_leds( data: u8) {

	command_send( 0xed, data, true);

}

pub fn set_scan_code_set( keymap: u8) {

	command_send( 0xf0, keymap, true);

}

/// Reset scan code data in driver.
pub fn check_scan_code_set() {

	command_send( 0xf0, 0, false);

}

/// Convert to PS/2 format and set typematic rate and delay.
pub fn set_typematic(repeat: u8, delay_before: u16) {
	let mut data = match repeat {

		_ => 0,

	};

	data |= match delay_before {

		..375 => 0,
		376..625 => 1,
		626..875 => 2,
		_ => 3,

	} << 4;
	set_formatted_typematic( data);

}

/// Set typematic rate and delay from PS/2 format.
pub fn set_formatted_typematic( data: u8) {

	command_send( 0xf3, data, true);

}

/// Append new command to queue.
fn command_send( comm: u8, data: u8, send_data: bool) {

	unsafe {

		if !COMMANDS.full() {

			match COMMANDS.push( Command { comm, data, state: ( send_data as u8) << 1, resend: 0}) {

				Ok( ()) => {},
				Err( ()) => {},

			};

		}

	}

}

fn can_write() -> bool {

	unsafe { inb( 0x64) & 0b10 == 0 }
}

fn can_read() -> bool {

	unsafe { inb( 0x64) & 1 != 0 }
}
