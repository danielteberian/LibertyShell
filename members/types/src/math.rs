use super::Value;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OpError
{
	TypeError,
	CalcError,
	ParseError(lexical::Error),
}

pub trait Pow<RHS = Self>
{
	type Output;
	fn pow(self, power: RHS) -> Self::Output;
}
pub trait EuclDiv<RHS = Self>
{
	type Output;
	fn eucl_div(self, rhs: RHS) -> Self::Output;
}

macro_rules! math
{
	($trait:ident, $fn:ident, $op_f_f:expr, $op_i_i:expr) =>
	{
		math!($trait, $fn, $op_f_f, $op_i_i, false);
	};
	($trait:ident, $fn:ident, $op_f_f:expr, $op_i_i:expr, $allfloat:expr) =>
	{
		impl<'a, T> $trait for &'a Value<T>
		{
			type Output = Result<Value<T>, OpError>;
			fn $fn(self, rhs: Self) -> Self::Output
			{
				if let Value::Str(rhs) = rhs
				{
					if $allfloat
					{
						lexical::parse::<f64, _>(rhs)
							.map_err(OpError::ParseError)
							.and_then(|rhs| self.$fn(rhs))
					}
					else
					{
						if let Ok(rhs) = lexical::parse::<i128, _>(rhs)
						{
							self.$fn(rhs)
						}
						else
						{
							lexical::parse::<f64, _>(rhs)
								.map_err(OpError::ParseError)
								.and_then(|rhs| self.$fn(rhs))
						}
					}
				}
				else
				{
					Err(OpError::TypeError)
				}
			}
		}
		impl<'a, T> $trait<Value<T>> for &'a Value<T>
		{
			type Output = Result<Value<T>, OpError>;
			fn $fn(self, rhs: Value<T>) -> Self::Output
			{
				self.$fn(&rhs)
			}
		}
		impl<'a, T> $trait<i128> for &'a Value<T>
		{
			type Output = Result<Value<T>, OpError>;
			fn $fn(self, rhs: i128) -> Self::Output
			{
				match self
				{
					Value::Str(lhs) => if $allfloat
					{
						lexical::parse::<f64, _>(lhs)
							.map_err(OpError::ParseError)
							.map(|lhs| lexical::to_string($op_f_f(lhs, rhs as f64)))
					}
					else
					{
