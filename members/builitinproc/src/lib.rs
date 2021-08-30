extern crate proc_macro;
use darling::{util::Flag, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use std::{fs::File, io::Write};

#[derive(Debug, FromMeta)]
struct MacroArguments
{
	#[darling(default)]
	names:	Option<String>,
	#[darling(rename = "man")]
	help:	String,
	#[darling(default)]
	authors:	Flag,
	#[darling(rename = "desc")]
	short_desc:	String,
}

