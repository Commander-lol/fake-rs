use helper::*;
use Fake;
use faker::{Company, Name};

pub trait FileName: Fake + Company + Name {
	 #[inline]
	fn name() {
		unimplemented!()
	}

	 #[inline]
	fn archive_name() {
		unimplemented!()
	}

	 #[inline]
	fn image_name() {
		unimplemented!()
	}

	 #[inline]
	fn audio_name() {
		unimplemented!()
	}

	 #[inline]
	fn video_name() {
		unimplemented!()
	}

	 #[inline]
	fn document_name() {
		unimplemented!()
	}


	 #[inline]
	fn extension() {
		unimplemented!()
	}

	 #[inline]
	fn archive_extension() {
		unimplemented!()
	}

	 #[inline]
	fn image_extension() {
		unimplemented!()
	}

	 #[inline]
	fn audio_extension() {
		unimplemented!()
	}

	 #[inline]
	fn video_extension() {
		unimplemented!()
	}

	 #[inline]
	fn document_extension() {
		unimplemented!()
	}

}