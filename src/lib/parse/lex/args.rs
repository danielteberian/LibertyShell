use thiserror::Error;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Comm
{
	Type1,
	Type2,
	None,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Field
{
	Proc,
	Array,
	Braces,
}
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lvl
{
	parens: u8,
	array: u8,
	braces: u8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
pub enum LvlErr
{
	#[error("[ERR] NO CLOSING PARENTHESE")]
	UnmatchedParen,
	#[error("[ERR] NO CLOSING BRACKET")]
	UnmatchedBracket,
	#[error("[ERR] NO CLOSING BRACE")]
	UnmatchedBrace,
	#[error("[ERR] CLOSING PARENTHESE(S) WITH NO CORRESPONDING OPEN PARENTHESE(S)")]
	ExtraParen,
	#[error("[ERR] CLOSING BRACKET(S) WITH NO CORRESPONDING OPEN BRACKET(S)")]
	ExtraBracket,
	#[error("[ERR] CLOSING BRACE(S) WITH NO CORRESPONDING OPEN BRACE(S)")]
	ExtraBrace,
}
impl Lvl
{
	pub fn up(&mut self, field: Field)
	{
		let lvl = match field
		{
			Field::Proc => &mut self.parens,
			Field::Array => &mut self.array,
			Field::Braces => &mut self.braces,
		};
		*lvl += 1;
	}
	pub fn down(&mut self, field: Field) -> Result<(), LvlErr>
	{
		let lvl = match field
		{
			Field::Proc if self.parens > 0 => &mut self.parens,
			Field::Array if self.array > 0 => &mut self.array,
			Field::Braces if self.braces > 0 => &mut self.braces,
			Field::Proc => return Err(LvlErr::ExtraParen),
			Field::Array => return Err(LvlErr::ExtraBracket),
			Field::Braces => return Err(LvlErr::ExtraBrace),
		};
		*lvl -= 1;
		Ok(())
	}
	pub const fn rooted(self) -> bool
	{
		self.parens == 0 && self.array == 0 && self.braces == 0
	}
