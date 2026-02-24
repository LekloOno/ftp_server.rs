//! A procedural macro to simplify implementing new Ftp server responses.

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, DeriveInput, Data, Meta, Lit, Fields,
};

const VAR_IDENT: &'static str = "ftp_res";
const CODE_IDENT: &'static str = "code";
const MSG_IDENT: &'static str = "message";

/// This procedural macro allows to implement new Ftp Responses enum variants
/// in a very clear and simple way.
/// 
/// It implements the `FtpResponse` trait automatically, using the provided arguments.
/// 
/// **#[ftp_res]**
/// * `code` - corresponds to the numerical code 1xx, 2xx, 3xx, 4xx, 5xx or 6xx of the response
/// and will be displayed first in the formated response.
/// 
/// * `message` - corresponds to a formatted message associated to the response, and will be
/// displayed after the `code` in the formated response.
/// 
/// Such message can use standard rust formating, typically to display fields values of the enum variant.
/// 
/// # Notes
/// The macro expects the `FtpResponse` trait to be accessible in the scope the macro is called in.
/// 
/// # Example
/// ```rs
/// use crate::response::FtpResponse;
/// use ftp_response_impl::FtpResponse;
/// 
/// #[derive(FtpResponse)]
/// pub enum SuccessResponse {
///     #[ftp_res(code = 220, message = "Service ready for new user. Welcome to the FTP server.")]
///     ServiceReady,
///     #[ftp_res(code = 227, message = "Entering passive mode ({h1},{h2},{h3},{h4},{p1},{p2})")]
///     EnteringPasv { h1: u8, h2: u8, h3: u8, h4: u8, p1: u8, p2: u8 },
/// }
/// ```
#[proc_macro_derive(FtpResponse, attributes(ftp_res))]
pub fn derive_ftp_response(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let enum_name = input.ident;

    let Data::Enum(data_enum) = input.data else {
        panic!("FtpResponse can only be derived for enums");
    };

    let arms = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        let attr = variant.attrs.iter()
            .find(|a| a.path().is_ident(VAR_IDENT))
            .expect(&format!("Missing #[{VAR_IDENT}(...)] attribute"));

        let meta = attr.parse_args_with(
            syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated
        ).unwrap();

        let mut code = None;
        let mut message = None;

        for item in meta {
            match item {
                Meta::NameValue(nv) if nv.path.is_ident(CODE_IDENT) => {
                    let syn::Expr::Lit(expr) = nv.value else {
                        continue;
                    };

                    if let Lit::Int(lit) = expr.lit {
                        code = Some(lit);
                    }
                    // Ideally, we could stop looking for attribute once they are found
                }
                Meta::NameValue(nv) if nv.path.is_ident(MSG_IDENT) => {

                    let syn::Expr::Lit(expr) = nv.value else {
                        continue;
                    };

                    if let Lit::Str(lit) = expr.lit {
                        message = Some(lit);
                    }
                }
                _ => {}
            }
        }

        let code = code.expect(&format!("Missing {CODE_IDENT}"));
        let message = message.expect(&format!("Missing {MSG_IDENT}"));

        match &variant.fields {
            Fields::Unit => {
                quote! {
                    #enum_name::#variant_name => {
                        write!(control, "{} {}\r\n", #code, #message)
                    }
                }
            }
            Fields::Named(fields) => {
                let names = fields.named.iter().map(|f| f.ident.as_ref().unwrap());

                quote! {
                    #enum_name::#variant_name { #( #names ),* } => {
                        write!(control, "{} ", #code)?;
                        write!(control, #message)?;
                        write!(control, "\r\n")
                    }
                }
            }
            _ => panic!("Only unit and named fields supported"),
        }
    });

    let expanded = quote! {
        impl FtpResponse for #enum_name {
            fn send(&self, control: &mut dyn std::io::Write) -> std::io::Result<()> {
                use std::io::Write;
                match self {
                    #( #arms ),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}