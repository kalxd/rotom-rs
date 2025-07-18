use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
	Attribute, Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident, parse_macro_input,
};

fn is_has_database_attr(attrs: &[Attribute]) -> bool {
	attrs.iter().any(|Attribute { meta, .. }| {
		meta.require_path_only()
			.map(|p| {
				let path_name = p.get_ident();
				let state_id = Ident::new("database", Span::call_site());
				Some(&state_id) == path_name
			})
			.unwrap_or(false)
	})
}

fn get_database_in_named_field(data: FieldsNamed) -> Option<TokenStream> {
	data.named.iter().find_map(|f| {
		if is_has_database_attr(&f.attrs) {
			let field_name = &f.ident;
			Some(quote! { #field_name })
		} else {
			None
		}
	})
}

fn get_database_in_uname_field(data: FieldsUnnamed) -> Option<TokenStream> {
	data.unnamed.iter().enumerate().find_map(|(idx, f)| {
		if is_has_database_attr(&f.attrs) {
			let raw_token: proc_macro::TokenStream = idx.to_string().parse().unwrap();
			let token = TokenStream::from(raw_token);
			Some(token)
		} else {
			None
		}
	})
}

fn get_database_field(data: Data) -> Option<TokenStream> {
	match data {
		Data::Struct(f) => match f.fields {
			Fields::Named(ns) => get_database_in_named_field(ns),
			Fields::Unnamed(us) => get_database_in_uname_field(us),
			_ => panic!("Database不允许unit成员！"),
		},
		_ => panic!("Database仅支持struct定义的数据结构！"),
	}
}

pub fn database_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let data_name = input.ident;
	let field_name = get_database_field(input.data).expect("未找到#[database]标识！");

	let ast = quote! {
		#[automatically_derived]
		impl<'p> sqlx::Executor<'p> for &#data_name {
			type Database = sqlx::Postgres;

			fn fetch_many<'e, 'q: 'e, E>(
				self,
				query: E
			) -> futures::stream::BoxStream<'e, Result<sqlx::Either<<Self::Database as sqlx::Database>::QueryResult, <Self::Database as sqlx::Database>::Row> ,sqlx::Error>>
			where
				'p: 'e,
				E: 'q + sqlx::Execute<'q, Self::Database>,
			{
				self.#field_name.fetch_many(query)
			}

			fn fetch_optional<'e, 'q: 'e, E>(
				self,
				query: E
			) -> futures::future::BoxFuture<'e, Result<Option<<Self::Database as sqlx::Database>::Row>, sqlx::Error>>
			where
				'p: 'e,
				E: 'q + sqlx::Execute<'q, Self::Database>,
			{
				self.#field_name.fetch_optional(query)
			}

			fn prepare_with<'e, 'q: 'e>(
				self,
				sql: &'q str,
				parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo]
			) -> futures::future::BoxFuture<'e, Result<<Self::Database as sqlx::Database>::Statement<'q>, sqlx::Error>>
			where
				'p: 'e,
			{
				self.#field_name.prepare_with(sql, parameters)
			}

			fn describe<'e, 'q: 'e>(
				self,
				sql: &'q str
			) -> futures::future::BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
			where
				'p: 'e,
			{
				self.#field_name.describe(sql)
			}
		}
	};

	ast.into()
}
