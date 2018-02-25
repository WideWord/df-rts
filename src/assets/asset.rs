use std::cell::RefCell;
use std::rc::Rc;

pub struct Asset<T> {
	pub asset: Rc<RefCell<T>>,
}

impl<T> Asset<T> {

	pub fn asset(asset: T) -> Asset<T> {
		Asset {
			asset: Rc::new(RefCell::new(asset)),
		}
	}

}

impl<T> Clone for Asset<T> {

	fn clone(&self) -> Asset<T> {
		Asset {
			asset: self.asset.clone(),
		}
	}

}
