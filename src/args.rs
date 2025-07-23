//! Argument parsing for the `axin` procedural macro.
//!
//! This module defines the structures and parsing logic for handling
//! the various parameters accepted by the `#[axin(...)]` attribute macro.

use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Block, Expr, Ident, Path, Stmt, Token,
};

/// Parameter name constants.
pub mod param_names {
    /// The "prologue" parameter name.
    pub const PROLOGUE: &str = "prologue";
    /// The "on_enter" parameter name.
    pub const ON_ENTER: &str = "on_enter";
    /// The "on_exit" parameter name.
    pub const ON_EXIT: &str = "on_exit";
    /// The "decorator" parameter name.
    pub const DECORATOR: &str = "decorator";

    /// All supported parameter names for error messages.
    pub const ALL_PARAMS: &[&str] = &[PROLOGUE, ON_ENTER, ON_EXIT, DECORATOR];
}

/// Function call specification supporting both simple paths and parameterized calls.
///
/// Represents function references in macro arguments, supporting:
/// - Simple function names: `my_function`
/// - Parameterized calls: `my_function("arg1", 42)`
#[derive(Clone)]
pub enum FunctionSpec {
    /// Simple function path without arguments
    Simple(Path),
    /// Function call with arguments
    WithArgs(Path, Punctuated<Expr, Token![,]>),
}

impl Parse for FunctionSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path: Path = input.parse()?;

        if input.peek(syn::token::Paren) {
            let content;
            parenthesized!(content in input);
            Ok(FunctionSpec::WithArgs(
                path,
                Punctuated::<Expr, Token![,]>::parse_terminated(&content)?,
            ))
        } else {
            Ok(FunctionSpec::Simple(path))
        }
    }
}

/// Collection of arguments for the [`axin`](crate::axin) macro.
///
/// Contains a comma-separated list of macro parameters such as
/// `prologue(...)`, `on_enter(...)`, `decorator(...)`, and `on_exit(...)`.
pub struct AxinArgs {
    pub args: Punctuated<AxinArg, Token![,]>,
}

/// Individual argument types supported by the [`axin`](crate::axin) macro.
///
/// Each variant represents a specific instrumentation feature:
/// - Prologue: Statements inserted at function start
/// - OnEnter: Function called before main function
/// - OnExit: Function called after main function
/// - Decorator: Function wrapper for the main function
pub enum AxinArg {
    /// `prologue(statement1; statement2; ...)`
    ///
    /// Statements to insert at the beginning of the function body.
    Prologue { stmts: Vec<Stmt> },
    /// `on_enter(function)` or `on_enter(function(args))`
    ///
    /// Function to execute before the main function.
    OnEnter { func: FunctionSpec },
    /// `on_exit(function)` or `on_exit(function(args))`
    ///
    /// Function to execute after the main function.
    OnExit { func: FunctionSpec },
    /// `decorator(function)` or `decorator(function(args))`
    ///
    /// Decorator function to wrap the main function.
    Decorator { func: FunctionSpec },
}

impl Parse for AxinArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args = Punctuated::parse_terminated(input)?;
        Ok(AxinArgs { args })
    }
}

impl Parse for AxinArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        parenthesized!(content in input);

        match name.to_string().as_str() {
            param_names::PROLOGUE => Ok(AxinArg::Prologue {
                stmts: content.call(Block::parse_within)?,
            }),
            param_names::ON_ENTER | param_names::ON_EXIT | param_names::DECORATOR => {
                let func: FunctionSpec = content.parse()?;
                match name.to_string().as_str() {
                    param_names::ON_ENTER => Ok(AxinArg::OnEnter { func }),
                    param_names::ON_EXIT => Ok(AxinArg::OnExit { func }),
                    param_names::DECORATOR => Ok(AxinArg::Decorator { func }),
                    _ => unreachable!(),
                }
            }
            _ => {
                let name_str = name.to_string();
                let supported_params = param_names::ALL_PARAMS.join(", ");
                Err(syn::Error::new_spanned(
                    name,
                    format!(
                        "Unsupported parameter: '{}'. Supported parameters are: {}",
                        name_str, supported_params
                    ),
                ))
            }
        }
    }
}
