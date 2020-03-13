use crate::TagVec;
use crate::BitField;
use std::hash::Hash;

/// Conveniences for creating expressions easily.
/// The idea is to wildcard import this module in
/// a confined scope, such that you get access to all the goodies
/// within, but only for a short while, so that it
/// doesn't reck the rest of your codebase.
pub mod expressions {
	use crate::query::Expression;
	use std::hash::Hash;

	/// Creates a tag expression
	pub fn tag<'a, Q>(tag: &'a Q) -> Expression<'a, Q>
			where Q: ?Sized + Eq + Hash {
		Expression::Tag(tag)
	}

	/// Creates an and expression
	pub fn and<'a, Q>(a: Expression<'a, Q>, b: Expression<'a, Q>) -> Expression<'a, Q>
			where Q: ?Sized + Eq + Hash {
		Expression::And(Box::new(a), Box::new(b))
	}

	/// Creates an or expression
	pub fn or<'a, Q>(a: Expression<'a, Q>, b: Expression<'a, Q>) -> Expression<'a, Q>
			where Q: ?Sized + Eq + Hash {
		Expression::Or(Box::new(a), Box::new(b))
	}

	/// Create a not expression
	pub fn not<'a, Q>(a: Expression<'a, Q>) -> Expression<'a, Q>
			where Q: ?Sized + Eq + Hash {
		Expression::Not(Box::new(a))
	}
}

/// A definition of a query
pub enum Expression<'a, Q> where Q: ?Sized + Hash + Eq + 'a {
	And(Box<Expression<'a, Q>>, Box<Expression<'a, Q>>),
	Or(Box<Expression<'a, Q>>, Box<Expression<'a, Q>>),
	Not(Box<Expression<'a, Q>>),
	Tag(&'a Q),
}

impl<'a, Q: ?Sized + Hash + Eq + 'a> Expression<'a, Q> {
	/// Returns the number of commands it takes
	/// to represent the command
	fn command_size(&self) -> usize {
		use Expression::*;

		match self {
			And(a, b) => 1 + a.command_size() + b.command_size(),
			Or(a, b) => 1 + a.command_size() + b.command_size(),
			Not(a) => 1 + a.command_size(),
			Tag(_) => 1,
		}
	}

	/// Converts this expression to a series of commands
	fn to_commands(self, tag_requests: &mut Vec<&'a Q>, commands: &mut Vec<u16>) {
		use Expression::*;

		match self {
			And(a, b) => {
				a.to_commands(tag_requests, commands);
				b.to_commands(tag_requests, commands);
				commands.push(QUERY_CMD_AND);
			},
			Or(a, b) => {
				a.to_commands(tag_requests, commands);
				b.to_commands(tag_requests, commands);
				commands.push(QUERY_CMD_OR);
			},
			Not(a) => {
				a.to_commands(tag_requests, commands);
				commands.push(QUERY_CMD_NOT);
			},
			Tag(tag) => {
				let id = tag_requests.len() as u16;
				assert!(id < QUERY_CMD_TAG);
				tag_requests.push(tag);
				// Any command with an id lower than QUERY_CMD_TAG is
				// a get tag command.
				commands.push(id);
			},
		}
	}
}

// Define command constants. This cannot be represented as an
// enum because the last property can be any value lower than QUERY_CMD_TAG
const QUERY_CMD_AND: u16 = 0xFFFF;
const QUERY_CMD_OR: u16 = 0xFFFD;
const QUERY_CMD_NOT: u16 = 0xFFFC;
const QUERY_CMD_TAG: u16 = 0xFFFC; // Less than, not equals

/// A Query iterator. Will iterate over the elements of a TagVec
/// that fulfill a requirement, defined by the "Expression" enum.
pub struct Query<'a, F> 
		where 
				F: BitField {
	tag_data: Vec<Option<&'a [F]>>,
	commands: Vec<u16>,
	bit_ctr: usize,
	total_bits: usize,
	data: F,
	stack: Vec<F>,
}

impl<'a, F> Query<'a, F> 
		where 
				F: BitField
{
	/// Creates a Query from an expression. This makes the representation of
	/// the query significantly more efficient(should be better at cache locality)
	/// The expressions are converted into "commands", and commands work like this:
	/// We have a list of commands. Each command is run once for every BitField in
	/// the BitField slices. These commands pop the data they need of the stack,
	/// and push the result. 
	/// The last command will give exactly one result left
	/// on the stack, and that result is the BitField for the next couple of
	/// elements that fulfill the requirement.
	pub(crate) fn create_from<T, Q>(vec: &'a TagVec<T, F>, expr: Expression<'a, Q>) 
			-> Query<'a, F> 
				where T: Eq + Hash + Clone + std::borrow::Borrow<Q>, 
						Q: ?Sized + Eq + Hash + 'a {
		let mut tag_requests = Vec::new();
		let mut commands = Vec::with_capacity(expr.command_size());

		expr.to_commands(&mut tag_requests, &mut commands);

		// Get references to the data storing the things
		// the commands want
		let tag_data: Vec<_> = tag_requests.into_iter()
				.map(|request| vec.tag_fields.get(request).map(|v| v.data())).collect();

		Query {
			tag_data,
			commands,
			bit_ctr: 0,
			total_bits: vec.len(),
			data: F::empty(),
			stack: Vec::new(),
		}
	}

	/// Assumes that there is another element.
	/// Also, only returns a bool, wether or not
	/// the condition was fulfilled the next iteration
	fn sloppy_next(&mut self) -> bool {
		let local_index = self.bit_ctr % F::n_bits();

		if local_index == 0 {
			// We are on a new bit! Evaluate the local BitField first
			let data_index = self.bit_ctr / F::n_bits();

			// We assume that the stack is always sufficiently populated, because the "to_commands"
			// function shouldn't generate commands that break this
			self.stack.clear();
			let stack = &mut self.stack;
			let commands = &self.commands;
			let tag_data = &self.tag_data;
			for cmd in commands.iter() {
				match *cmd {
					QUERY_CMD_AND => {
						let a = stack.pop().unwrap();
						let b = stack.pop().unwrap();
						stack.push(a & b);
					},
					QUERY_CMD_OR => {
						let a = stack.pop().unwrap();
						let b = stack.pop().unwrap();
						stack.push(a | b);
					},
					QUERY_CMD_NOT => {
						let a = stack.pop().unwrap();
						stack.push(!a);
					},
					tag => {
						// It's definitely a tag
						stack.push(tag_data[tag as usize].map_or(F::empty(), |v| v[data_index]));
					},
				}
			}

			self.data = stack[0];
		}

		self.bit_ctr += 1;
		self.data.get_bit(local_index)
	}
}

impl<'a, F> Iterator for Query<'a, F> where F: BitField {
	type Item = usize;

	fn next(&mut self) -> Option<usize> {
		while self.bit_ctr < self.total_bits {
			if self.sloppy_next() {
				// The bit_ctr has been increased in sloppy_next, so 
				// we have to return the previous one
				return Some(self.bit_ctr - 1);
			}
		}

		None
	}
}
