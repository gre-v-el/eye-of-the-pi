use crate::darts::Darts;

pub enum State {
	Menu,
	Darts(Darts),
}
