use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
	Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident, Type, TypePath, parse_macro_input,
};

fn is_app_state_type(ty: &Type) -> bool {
	match ty {
		Type::Path(TypePath { path, .. }) => path.is_ident("AppState"),
		_ => false,
	}
}

fn handle_named_field(field: &FieldsNamed) -> TokenStream {
	let extract_ast = field.named.iter().map(|f| {
			let field_name = f.ident.as_ref().unwrap();
			let field_type = &f.ty;

			if is_app_state_type(&field_type) {
				quote! {
					let #field_name = req.app_state::<#field_type>()
                    .ok_or(Self::Error::Internal("无法正确获取state！".into()))?
                    .clone();
				}
			} else {
				quote! {
					let #field_name = <#field_type as ntex::web::FromRequest<E>>::from_request(req, _payload).await?;
				}
			}
		});

	let def_ast = field.named.iter().map(|f| {
		let field_name = f.ident.as_ref().unwrap();

		quote! {
			#field_name: #field_name
		}
	});

	let ast = quote! {
		#(#extract_ast)*

		let state = Self {
			#(#def_ast),*
		};

		Ok(state)
	};

	ast
}

fn handle_uname_field(field: &FieldsUnnamed) -> TokenStream {
	let extract_ast = field.unnamed.iter().enumerate().map(|(i, f)| {
			let field_type = &f.ty;
			let field_name = Ident::new(&format!("state_{i}"), Span::call_site());

			if is_app_state_type(field_type) {
				quote! {
					let #field_name = req.app_state::<#field_type>()
                    .ok_or(Self::Error::Internal("无法正确获取state！".into()))?
                    .clone();
				}
			}
			else {
				quote! {
					let #field_name = <#field_type as ntex::web::FromRequest<E>>::from_request(req, _payload).await?;
				}
			}
		});

	let def_ast = field.unnamed.iter().enumerate().map(|(i, _)| {
		let field_name = Ident::new(&format!("state_{i}"), Span::call_site());
		quote! {
			#field_name,
		}
	});

	quote! {
		#(#extract_ast)*

		Ok(Self(#(#def_ast)*))
	}
}

fn handle_struct_field(field: &Data) -> TokenStream {
	match field {
		Data::Struct(f) => match &f.fields {
			Fields::Named(name_field) => handle_named_field(name_field),
			Fields::Unnamed(uname_field) => handle_uname_field(uname_field),
			_ => panic!(""),
		},
		_ => panic!("AppEnv仅支持struct定义的数据结构！"),
	}
}

pub fn state_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let data_name = input.ident;
	let def = handle_struct_field(&input.data);

	let ast = quote! {
		impl<E: ntex::web::ErrorRenderer> ntex::web::FromRequest<E> for #data_name {
			type Error = crate::data::error::Error;

			async fn from_request(req: &ntex::web::HttpRequest, _payload: &mut ntex::http::Payload) -> Result<Self, Self::Error> {
				#def
			}
		}
	};

	proc_macro::TokenStream::from(ast)
}
