pub trait DriverDelay {

	fn wait( &mut self);
	fn increment_delay( &mut self);

}
