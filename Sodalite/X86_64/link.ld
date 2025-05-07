ENTRY( _start)

PHDRS {

	text   PT_LOAD FLAGS( 5);
	data   PT_LOAD FLAGS( 6);
	rodata PT_LOAD FLAGS( 4);

}

MEMORY {

	sodalite (rw) : ORIGIN = 0x7e00, LENGTH = 480K

}

SECTIONS {

	. = 0x7e00;

	.text.early_entry : {

		KEEP( *( .text.early_entry))

	} > sodalite :text

	.text   : {} > sodalite :text
	.data   : {} > sodalite :data
	.rodata : {} > sodalite :rodata
	.bss    : {} > sodalite :data

	/DISCARD/ : {

		*( .note.gnu.property)

	}

}
