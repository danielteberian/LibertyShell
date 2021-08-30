use std::{
	borrow::Borrow,
	collections::HashMap,
	hash::Hash,
	ops::{Deref, DerefMut},
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Namespc
{
	Global,
	Specific(usize),
	Any,
}

#[derive(Clone, Debug)]
pub struct Scopes<K: Hash + Eq, V>
{
	flags: u8,
	scopes: Vec<Scope<K, V>>,
	current: usize,
}

#[derive(Clone, Debug)]
pub struct Scope<K: Hash + Eq, V>
{
	vars: HashMap<K, V>,
	namespc: bool,
}

impl<K: Hash + Eq, V> Deref for Scope<K, V>
{
	type Target = HashMap<K, V>;
	fn deref(&self) -> &Self::Target
	{
		&self.vars
	}
}

impl<K: Hash +  Eq, V> DerefMut for Scope<K, V>
{
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.vars
	}
}

impl<K: Hash + Eq, V: Clone> Scopes<K, V>
{
	pub fn with_capacity(cap: usize) -> Self
	{
		Self
		{
			flags: 0,
			scopes: vec![Scope { vars: HashMap::with_capacity(cap), namespc: false }],
			current: 0,
		}
	}

	pub fn new_scope(&mut self, namespc: bool)
	{
		self.current += 1;
		if self.current >= self.scopes.len()
		{
			self.scopes.push(Scope { vars: HashMap::with_capacity(64), namespc });
		}
		else
		{
			self.scopes[self.current].namespc = namespc;
		}
	}

	pub fn pop_scope(&mut self)
	{
		self.scopes[self.current].clear();
		self.current -= 1;
	}

	pub fn pop_scopes(&mut self, idx: usize) -> impl Iterator<Item = Scope<K, V>> + '_
	{
		self.current = idx;
		self.scopes.drain(idx + 1..)
	}

	pub fn append_scopes(&mut self, scopes: Vec<Scope<K, V>>)
	{
		self.scopes.drain(self.current + 1..);
		self.current += scopes.len();
		self.scopes.extend(scopes);
	}

	pub fn scopes(&self) -> impl DoubleEndedIterator<Item = &Scope<K, V>>
	{
		let amt = self.scopes.len() - self.current - 1;
		self.scopes.iter().rev().skip(amt)
	}

	pub fn scopes_mut(&mut self) -> impl Iterator<Item = &mut Scope<K, V>>
	{
		let amt = self.scopes.len() - self.current - 1;
		self.scopes.iter_mut().rev().skip(amt)
	}

	pub fn index_scope_for_var<Q: ?Sized>(&self, name: &Q) -> Option<usize>
	where
		K: Borrow<Q>,
		Q: Hash + Eq,
	{
		let amt = self.scopes.len() - self.current - 1;
		for (i, scope) in self.scopes.iter().enumerate().rev().skip(amt)
		{
			if scope.contains_key(name)
			{
				return Some(i);
			}
		}
		None
	}

	pub fn set<T: Into<K>, S: Into<V>>(&mut self, name: T, value: S) -> Option<V>
	{
		self.scopes[self.current].insert(name.into(), value.into())
	}

	pub fn get<Q: ?Sized>(&self, name: &Q, namespc: Namespc) -> Option<&V>
	where
		K: Borrow<Q>,
		Q: Hash + Eq,
	{
		match namespc
		{
			Namespc::Global => self
				.scopes()
				.rev()
				.take_while(|scope| !scope.namespc)
				.filter_map(|scope| scope.get(name))
				.last(),
			Namespc::Specific(mut up) =>
			{
				for scope in self.scopes()
				{
					if up == 0
					{
						if let val @ Some(_) = scope.get(name)
						{
							return val;
						}
						else if scope.namespc
						{
							return None;
						}
					}
					else if scope.namespc
					{
						up -= 1;
					}
				}
				None
			}
			Namespc::Any => self.scopes().find_map(|scope| scope.get(name)),
		}
	}
	pub fn get_mut<Q: ?Sized>(&mut self, name: &Q) -> Option<&mut V>
	where
		K: Borrow<Q>,
		Q: Hash + Eq,
	{
		for scope in self.scopes_mut()
		{
			let exit = scope.namespc;
			if let val @ Some(_) = scope.get_mut(name)
			{
				return val;
			}
			if exit
			{
				break;
			}
		}
		None
	}
	pub fn remove_variable<Q: ?Sized>(&mut self, name: &Q) -> Option<V>
	where
		K: Borrow<Q>,
		Q: Hash + Eq,
	{
		for scope in self.scopes_mut()
		{
			let exit = scope.namespc;
			if let val @ Some(_) = scope.remove(name)
			{
				return val;
			}
			if exit
			{
				break;
			}
		}
		None
	}
}
