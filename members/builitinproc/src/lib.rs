extern crate proc_macro;
use darling::{util::Flag, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use std::{fs::File, io::Write};

#[derive(Debug, FromMeta)]
struct MacroArgs
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

#[proc_macro_attribute]
pub fn builtin(attr: TokenStream, item: TokenStream) -> TokenStream
{
	let input = syn::parse_macro_input!(item as syn::ItemFn);
	let attrs = syn::parse_macro_input!(attr as syn::AttributeArgs);
	let syn::ItemFn { vis, sig, block, .. } = &input;
	let syn::Signature { ident, fn_token, inputs, output, .. } = sig;
	let args = match MacroArgs::from_list(&attrs)
	{
		Ok(v) => v,
		Err(e) => return e.write_errors().into(),
	};
	let name = quote::format_ident!("BUILTIN_{}", &ident, span = ident.span());
	let help = args.help.trim();
	let names = args.names.unwrap_or_else(|| ident.to_string());
	let bugs = "BUGS If you encounter a bug, open an issue on the repository, and I will do my best to patch said bug out of the next release: https://github.com/danielteberian/LibertyShell";
	let extra = "who cares".to_string();
	let man = format!(
		"NAME\n	{names}	-  {short_desc}\n\n{help}\n\n{help}\n\n{bugs}{extra}",
		names = names,
		short_desc = args.short_desc,
		help = help,
		bugs = bugs,
		extra = if args.authors.is_none() { &extra } else { "" },
	);
	let help = format!("{} - {}\n\n'''txt\n{}\n'''", names, args.short_desc, help);

	if cfg!(feature = "man")
	{
		let mut man = File::create(format!("manual/builtins/{}.1", &ident)).unwrap();
		man.write_all(help.as_bytes()).unwrap();
	}

	let result = quote!
	{
		#[doc = #help]
		#vis #fn_token #name(#inputs) #output
		{
			if libertyshell::builtins::man_pages::check_hel(args, #man)
			{
				return libertyshell::builtins::Status::SUCCESS;
			}
			#block
		}
	};
	result.into()
}

