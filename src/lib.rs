extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote, TokenStreamExt};

#[proc_macro]
pub fn gen_pub_mod(_item: TokenStream) -> TokenStream {
    let mut q = quote! {};

    q.append_all((1..26).map(|num: u8| {
        let num_str = format!("{:02}", num);
        let name = format_ident!("d{}", num_str);
        quote! {
                pub mod #name;
        }
    }));
    //println!("{}", q.to_string());
    q.into()
}

#[proc_macro]
pub fn gen_module_map(_item: TokenStream) -> TokenStream {
    let mut q = quote! {};

    q.append_all((1..26).map(|num: u8| {
        let num_str = format!("{:02}", num);
        let name = format_ident!("d{}", num_str);
        //let num_ident = format_ident!("{}", num_str);

        quote! {
            if days::#name::IMPLEMENTED {
                res.insert(
                    #num,
                    DayInfo {
                        p1: days::#name::p1,
                        p2: days::#name::p2,
                        interactive: days::#name::INTERACTIVE,
                        filename: format!("days/{}/input.txt", stringify!(#name)),
                    },
                );
            }
        }
    }));
    //println!("{}", q.to_string());
    q.into()
}
