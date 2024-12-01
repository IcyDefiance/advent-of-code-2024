use indicatif::ProgressBar;
use std::{borrow::Cow, time::Duration};

pub fn using_spinner<T>(message: impl Into<Cow<'static, str>>, f: impl FnOnce() -> T) -> T {
	let pb = ProgressBar::new_spinner().with_message(message);
	pb.enable_steady_tick(Duration::from_millis(100));

	let res = f();

	pb.finish_and_clear();

	res
}
