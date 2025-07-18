use proc_macro::TokenStream;

mod database;
mod state;

#[proc_macro_derive(State)]
pub fn state_derive_macro(input: TokenStream) -> TokenStream {
	state::state_derive(input)
}

#[proc_macro_derive(Database, attributes(database))]
pub fn database_derive_macro(input: TokenStream) -> TokenStream {
	database::database_derive(input)
}
