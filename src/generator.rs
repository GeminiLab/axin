//! Code generation for the `axin` procedural macro.
//!
//! This module contains the logic for transforming annotated functions
//! according to the specified instrumentation parameters.

use crate::args::{AxinArg, FunctionSpec};
use proc_macro2::Span;
use quote::quote;
use syn::{parse_quote, FnArg, Ident, ItemFn, Pat, Stmt, Token};

/// Generate the enhanced function with the specified instrumentation features.
///
/// Transforms the original function by adding prologue statements, entry/exit hooks,
/// and decorator wrapping according to the provided parameters.
///
/// ## Parameters
///
/// - `input_fn`: The original function to be enhanced
/// - `prologue_stmts`: Statements to insert at function start
/// - `decorator_fn`: Optional decorator function specification
/// - `on_enter_funcs`: List of entry hook function specifications
/// - `on_exit_funcs`: List of exit hook function specifications
///
/// ## Returns
///
/// Token stream representing the transformed function code.
pub fn generate_enhanced_function(
    input_fn: ItemFn,
    prologue_stmts: Vec<Stmt>,
    decorator_fn: Option<FunctionSpec>,
    on_enter_funcs: Vec<FunctionSpec>,
    on_exit_funcs: Vec<FunctionSpec>,
) -> proc_macro2::TokenStream {
    let original_fn = input_fn.clone();
    let fn_vis = &original_fn.vis;
    let fn_sig = &original_fn.sig;
    let fn_inputs = &fn_sig.inputs;
    let fn_output = &fn_sig.output;
    let original_block = original_fn.block;

    // Build the argument list for the inner original function
    let args: Vec<_> = fn_inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    Some(&pat_ident.ident)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    // Build the inner function body
    let mut inner_stmts = Vec::new();
    inner_stmts.extend(prologue_stmts);
    inner_stmts.extend(original_block.stmts);

    // Build the final function body
    let mut final_stmts = Vec::new();

    // Add on_enter calls (in order)
    for on_enter in &on_enter_funcs {
        let call_expr = generate_function_call(on_enter);
        final_stmts.push(parse_quote! { #call_expr; });
    }

    // Define the inner original function
    final_stmts.push(parse_quote! {
        let original_fn = |#fn_inputs| #fn_output {
            #(#inner_stmts)*
        };
    });

    // Call decorator or directly call the original function
    if let Some(decorator) = &decorator_fn {
        let decorator_call = generate_decorator_call(decorator, &args);
        final_stmts.push(parse_quote! {
            let __result = #decorator_call;
        });
    } else {
        final_stmts.push(parse_quote! {
            let __result = original_fn(#(#args),*);
        });
    }

    // Add on_exit calls (in order)
    for on_exit in &on_exit_funcs {
        let call_expr = generate_function_call(on_exit);
        final_stmts.push(parse_quote! { #call_expr; });
    }

    // Always return the result, even if it's `()`
    final_stmts.push(parse_quote! {
        return __result;
    });

    // Build the final function
    let final_block = syn::Block {
        brace_token: original_block.brace_token,
        stmts: final_stmts,
    };

    quote! {
        #fn_vis #fn_sig #final_block
    }
}

/// Generate function call expression from a function specification.
///
/// Converts a `FunctionSpec` into the appropriate function call token stream,
/// handling both simple function calls and calls with arguments.
fn generate_function_call(func_spec: &FunctionSpec) -> proc_macro2::TokenStream {
    match func_spec {
        FunctionSpec::Simple(path) => {
            quote! { #path() }
        }
        FunctionSpec::WithArgs(path, args) => {
            quote! { #path(#args) }
        }
    }
}

/// Generate decorator call expression for wrapping the original function.
///
/// Creates the appropriate call pattern for decorator functions, handling both
/// simple decorators and parameterized decorators. The original function arguments
/// are passed through to maintain the function signature.
fn generate_decorator_call(
    func_spec: &FunctionSpec,
    orig_args: &[&Ident],
) -> proc_macro2::TokenStream {
    match func_spec {
        FunctionSpec::Simple(path) => {
            if orig_args.is_empty() {
                quote! { #path(original_fn) }
            } else {
                quote! { #path(original_fn, #(#orig_args),*) }
            }
        }
        FunctionSpec::WithArgs(path, args) => {
            // For Path(args), we call Path(args)(original_function, ...)
            if orig_args.is_empty() {
                quote! { (#path(#args))(original_fn) }
            } else {
                quote! { (#path(#args))(original_fn, #(#orig_args),*) }
            }
        }
    }
}

/// Process and extract components from attribute arguments.
///
/// Parses the macro arguments and separates them into their respective components:
/// prologue statements, decorator specification, entry functions, and exit functions.
///
/// ## Returns
///
/// A tuple containing:
/// - `Vec<Stmt>`: Prologue statements to insert
/// - `Option<FunctionSpec>`: Decorator function specification
/// - `Vec<FunctionSpec>`: List of entry hook function specifications  
/// - `Vec<FunctionSpec>`: List of exit hook function specifications
pub fn process_attribute_args(
    attribute_args: crate::args::AxinArgs,
) -> (
    Vec<Stmt>,
    Option<FunctionSpec>,
    Vec<FunctionSpec>,
    Vec<FunctionSpec>,
) {
    let mut prologue_stmts: Vec<Stmt> = Vec::new();
    let mut decorator_fn: Option<FunctionSpec> = None;
    let mut on_enter_funcs: Vec<FunctionSpec> = Vec::new();
    let mut on_exit_funcs: Vec<FunctionSpec> = Vec::new();

    for arg in attribute_args.args.into_iter() {
        match arg {
            AxinArg::Prologue { stmts } => {
                for stmt in stmts {
                    if let syn::Stmt::Expr(expr, None) = stmt {
                        // Convert expression to statement
                        prologue_stmts
                            .push(syn::Stmt::Expr(expr, Some(Token![;](Span::call_site()))));
                    } else {
                        // Use other types of statements directly
                        prologue_stmts.push(stmt);
                    }
                }
            }
            AxinArg::OnEnter { funcs } => {
                // Add all functions from this on_enter declaration
                on_enter_funcs.extend(funcs.list);
            }
            AxinArg::OnExit { funcs } => {
                // Add all functions from this on_exit declaration
                on_exit_funcs.extend(funcs.list);
            }
            AxinArg::Decorator { func } => {
                decorator_fn = Some(func);
            }
        }
    }

    (prologue_stmts, decorator_fn, on_enter_funcs, on_exit_funcs)
}
