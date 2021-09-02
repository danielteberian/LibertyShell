use super::Index;
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Range
{
	start: Index,
	end: Index,
	inclusive: bool,
}

impl Range
{
	pub fn bounds(&self, veclen: usize) -> Option<(usize, usize)>
	{
		let start = self.start.resolve(veclen)?;
		let end = self.end.resolve(veclen)?;
		if end < start
		{
			None
		}
		else if self.inclusive
		{
			Some((start, end - start + 1))
		}
		else
		{
			Some((start, end - start))
		}
	}
	pub fn exclusive(start: Index, end: Index) -> Range
	{
		Range
		{
			start, end, inclusive: false
		}
	}
	pub fn inclusive(start: Index, end: Index) -> Range
	{
		Range
		{
			start, end, inclusive: true
		}
	}
	pub fn from(start: Index) -> Range
	{
		Range
		{
			start, end: Index::new(-1), inclusive: true
		}
	}
	pub fn to(end: Index) -> Range
	{
		Range
		{
			start: Index::new(0), end, inclusive: false
		}
	}
}
