#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Index
{
	Forward(usize),
	Backward(usize),
}
impl Index
{
	pub fn resolve(&self, veclen: usize) -> Option<usize>
	{
		match *self
		{
			Index::Forward(n) => Some(n),
			Index::Backward(n) if n >= veclen => None,
			Index::Backward(n) => Some(veclen - (n + 1)),
		}
	}
	pub fn new(input: isize) -> Index
	{
		if input < 0
		{
			Index::Backward((input.abs() as usize) - 1)
		}
		else
		{
			Index::Forward(input.abs() as usize)
		}
	}
}
