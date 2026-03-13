use dotenvy::{EnvLoader, EnvSequence};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::io;
use syn::{parse::Parse, Ident, LitBool, LitStr, Token};

struct DotenvInput {
    path: Option<LitStr>,
    override_: bool,
    var_name: LitStr,
}

impl Parse for DotenvInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut path = None;
        let mut var_name = None;
        let mut override_ = None;

        while !input.is_empty() {
            if let Ok(ident) = input.parse::<Ident>() {
                input.parse::<Token![=]>()?;
                match ident.to_string().as_str() {
                    "path" => {
                        if path.is_some() {
                            return Err(syn::Error::new(
                                ident.span(),
                                "each attribute must be set only once",
                            ));
                        }
                        path = Some(input.parse::<LitStr>()?);
                    }
                    "override_" => {
                        if override_.is_some() {
                            return Err(syn::Error::new(
                                ident.span(),
                                "each attribute must be set only once",
                            ));
                        }
                        override_ = Some(input.parse::<LitBool>()?.value);
                    }
                    "var" => {
                        if var_name.is_some() {
                            return Err(syn::Error::new(
                                ident.span(),
                                "variable name must be set only once",
                            ));
                        }
                        var_name = Some(input.parse::<LitStr>()?);
                    }
                    _ => {
                        return Err(syn::Error::new(
                            ident.span(),
                            format!("unkown attribute: {ident}"),
                        ))
                    }
                }
            } else if let Ok(s) = input.parse::<LitStr>() {
                if var_name.is_some() {
                    return Err(syn::Error::new(
                        s.span(),
                        "unexpected token in macro input (variable name must be set only once)",
                    ));
                }
                var_name = Some(s);
            } else {
                return Err(syn::Error::new(
                    input.span(),
                    "unexpected token in macro input",
                ));
            }
            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        let Some(var_name) = var_name else {
            return Err(syn::Error::new(
                input.span(),
                "environment variable name not defined",
            ));
        };

        Ok(Self {
            path,
            override_: override_.unwrap_or(true),
            var_name,
        })
    }
}

/// Loads an environment variable from a file at compile time.
///
/// This macro will expand to the value of the named environment variable at compile
/// time, yielding an expression of type `&'static str`. Use
/// [`dotenvy::var`] instead if you want to read the value at runtime.
///
/// If the environment variable is not defined, then a compilation error will be
/// emitted. To not emit a compile error, use the [`option_dotenv!`](option_dotenv)
/// macro instead. A compilation error will also be emitted if the environment
/// variable is not a valid Unicode string or if reading the .env file failed.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust ignore
///     # use dotenvy_macro::dotenv;
///     assert_eq!(dotenv!("VARIABLE_1"), "value");
/// ```
///
/// ```rust compile_fail
///     # use dotenvy_macro::dotenv;
///     const UNSET_VAR: &str = dotenv!("UNSET_VAR");
/// ```
///
/// Custom attributes:
///
/// ```rust ignore
///     # use dotenvy_macro::dotenv;
///     // Does not override current env with .env contents
///     const NOT_OVERRIDEN: &str = dotenv!("VARIABLE_1", override_ = false);
///     // Reads from custom file path
///     const CUSTOM_PATH: &str = dotenv!("VARIABLE_PROD", path = ".env.prod");
///     // Specifying the `var` attribute
///     const CUSTOM_PATH: &str = dotenv!(var = "VARIABLE_1");
/// ```
#[proc_macro]
pub fn dotenv(input: TokenStream) -> TokenStream {
    let input = input.into();
    dotenv_inner(input).into()
}

fn dotenv_inner(input: TokenStream2) -> TokenStream2 {
    let args = match syn::parse2::<DotenvInput>(input) {
        Ok(a) => a,
        Err(e) => return e.into_compile_error(),
    };

    let sequence = if args.override_ {
        EnvSequence::EnvThenInput
    } else {
        EnvSequence::InputThenEnv
    };

    let path = args.path.map_or_else(|| "./.env".to_owned(), |p| p.value());
    let var_name = args.var_name.value();

    let loader = EnvLoader::new().path(&path).sequence(sequence).load();

    match loader.as_ref().map(|l| l.get(&var_name)) {
        Ok(Some(v)) => quote!(#v),
        Ok(None) => {
            let msg = format!("environment variable `{var_name}` not set");
            quote! {
                compile_error!(#msg)
            }
        }
        Err(e) => {
            if let dotenvy::Error::Io(ioe, _) = e {
                if ioe.kind() == io::ErrorKind::NotFound {
                    if let Ok(var) = std::env::var(&var_name) {
                        return quote!(#var);
                    } else {
                        return quote! {
                            compile_error!("environment variable not set and env file missing")
                        };
                    }
                }
            }
            let msg = e.to_string();
            quote! {
                compile_error!(#msg)
            }
        }
    }
}

/// Optionally loads an environment variable from a file at compile time.
///
/// If the named environment variable is present at compile time, this will expand
/// into an expression of type `Option<&'static str>` whose value is `Some` of the
/// value of the environment variable (a compilation error will be emitted if the
/// environment variable is not a valid Unicode string or if reading the .env file
/// failed). If either the environment variable or the .env file is not present,
/// then this will expand to `None`. Use [`dotenvy::var`] instead if
/// you want to read the value at runtime.
///
/// A compile time error is only emitted when using this macro if the environment
/// variable exists and is not a valid Unicode string or if reading the .env file
/// failed. To also emit a compile error if the environment variable is not present,
/// use the [`dotenv!`](dotenv) macro instead.
///
/// # Examples
///
/// Basic usage:
///
/// ```rust no_run
///     # use dotenvy_macro::option_dotenv;
///     assert_eq!(option_dotenv!("UNSET_VAR"), None);
///     assert_eq!(option_dotenv!("SET_VAR"), Some("value"));
/// ```
///
/// Custom attributes:
///
/// ```rust no_run
///     # use dotenvy_macro::option_dotenv;
///     // Does not override current env with .env contents
///     const NOT_OVERRIDEN: Option<&str> = option_dotenv!("VARIABLE_1", override_ = false);
///     // Reads from custom file path
///     const CUSTOM_PATH: Option<&str> = option_dotenv!("VARIABLE_PROD", path = ".env.prod");
///     // Specifying the `var` attribute
///     const VAR_ATTR: Option<&str> = option_dotenv!(var = "VARIABLE_1");
/// ```
#[proc_macro]
pub fn option_dotenv(input: TokenStream) -> TokenStream {
    let input = input.into();
    option_dotenv_inner(input).into()
}

fn option_dotenv_inner(input: TokenStream2) -> TokenStream2 {
    let args = match syn::parse2::<DotenvInput>(input) {
        Ok(a) => a,
        Err(e) => return e.into_compile_error(),
    };

    let sequence = if args.override_ {
        EnvSequence::EnvThenInput
    } else {
        EnvSequence::InputThenEnv
    };

    let path = args.path.map_or_else(|| "./.env".to_owned(), |p| p.value());
    let var_name = args.var_name.value();

    let loader = EnvLoader::new().path(&path).sequence(sequence).load();

    match loader.as_ref().map(|l| l.get(&var_name)) {
        Ok(Some(v)) => quote!(Some(#v)),
        Ok(None) => quote!(None::<&str>),
        Err(e) => {
            if let dotenvy::Error::Io(ioe, _) = e {
                if ioe.kind() == io::ErrorKind::NotFound {
                    if let Ok(var) = std::env::var(&var_name) {
                        return quote!(Some(#var));
                    }
                    return quote!(None::<&str>);
                }
            }
            let msg = e.to_string();
            quote! {
                compile_error!(#msg)
            }
        }
    }
}
