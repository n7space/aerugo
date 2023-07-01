#![feature(proc_macro_tracked_env)]

extern crate proc_macro;

use proc_macro::{tracked_env, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse2, parse_str, Expr, ExprLit, ItemConst, Lit};

#[proc_macro_attribute]
pub fn read_env(attr: TokenStream, item: TokenStream) -> TokenStream {
    read_env_impl(attr.into(), item.into()).into()
}

pub(crate) fn read_env_impl(attr: TokenStream2, item: TokenStream2) -> TokenStream2 {
    if let Ok(mut item_const) = parse2::<ItemConst>(item.clone()) {
        let name = get_variable_name(&item_const, attr);
        let value = match tracked_env::var(&name).ok() {
            Some(val) => val,
            None => return item,
        };

        let value_expr = create_value_expr(&name, &value, &item_const.expr);
        item_const.expr = Box::new(value_expr);

        quote!(#item_const)
    } else {
        panic!("env-parser supports only const items.")
    }
}

fn get_variable_name(item_const: &ItemConst, attr: TokenStream2) -> String {
    if attr.is_empty() {
        return format!("{}", item_const.ident);
    }

    let literal = match parse2(attr).expect("Unable to parse attribute.") {
        Expr::Lit(literal) => literal,
        _ => panic!("Attribute expression was not a literal."),
    };

    match &literal.lit {
        Lit::Str(str_literal) => str_literal.value(),
        _ => panic!("Environment variable name was not String."),
    }
}

fn create_value_expr(name: &str, value: &str, expr: &Expr) -> Expr {
    let literal = match expr {
        Expr::Lit(literal) => literal,
        _ => panic!("Default value expression was not a literal."),
    };

    let value_literal = match &literal.lit {
        Lit::Int(_) => parse_str(value).unwrap_or_else(|_| {
            panic!(
                "unable to parse value of variable {} to integer: {}",
                name, value
            )
        }),
        _ => panic!("env-parser support only integers."),
    };

    ExprLit {
        attrs: literal.attrs.clone(),
        lit: value_literal,
    }
    .into()
}
