#[macro_export]
macro_rules! genModuleMap {
    () => {
        use quote::quote;

        for i in (01..25) {
            let name = format!("d{:02}", i);
            let gen = quote! {
                    println!("days/{}/input.txt", stringify!(#name));
                };
            }
            gen.into();
        }
    }
