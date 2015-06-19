use std::fmt::Debug;

pub trait Manipulation<T, U> {
	fn arg_ins(&mut self, T, U, T);
	fn arg_mod(&mut self, T, U, T) -> T;
}

pub trait Navigation<T> {
	fn uses_of(&self, T) -> Vec<T>;
	fn args_of(&self, T) -> Vec<T>;
}

pub trait NavigationInternal<T> {
	// add param for manual aux
	fn add_uses_to(&self, T, &mut Vec<T>);
	fn add_args_to(&self, T, &mut Vec<T>);
}

impl<T, U> Navigation<T> for U where U: NavigationInternal<T> {
	fn uses_of(&self, t: T) -> Vec<T> {
		let mut r = Vec::<T>::new();
		self.add_uses_to(t, &mut r);
		return r
	}
	fn args_of(&self, t: T) -> Vec<T> {
		let mut r = Vec::<T>::new();
		self.add_args_to(t, &mut r);
		return r
	}
}

pub trait InstructionType: Debug {
	type PhiType: Copy + Clone + 'static;
	fn make_phi(Self::PhiType) -> Self;
	fn is_phi(&self) -> bool;
}
