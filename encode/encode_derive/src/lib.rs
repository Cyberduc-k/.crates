use quote::quote;
use syn::spanned::Spanned;

#[proc_macro_derive(Encode)]
pub fn derive_encode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let datatype = syn::parse_macro_input!(input as syn::DeriveInput);
    let type_name = &datatype.ident;
    let generics = &datatype.generics;
    let mut where_clause = quote!{};
    let body = match datatype.data {
        syn::Data::Struct(data) => {
            for field in &data.fields {
                let ty = &field.ty;
                
                where_clause = quote!{ #where_clause #ty: ::encode::Encode, };
            }
            
            let lines = data.fields.iter().enumerate().map(|(i, field)| {
                let ty = &field.ty;
                let name = match &field.ident {
                    Some(name) => quote!{ #name },
                    None => {
                        let lit = proc_macro2::Literal::usize_unsuffixed(i);

                        quote!{ #lit }
                    },
                };
                
                quote!{ <#ty as ::encode::Encode>::encode(&self.#name, e)?; }
            });
            
            quote!{ #(#lines)* }
        },
        syn::Data::Enum(data) => {
            for variant in &data.variants {
                for field in &variant.fields {
                    let ty = &field.ty;
                    
                    where_clause = quote!{ #where_clause #ty: ::encode::Encode, };
                }
            }
            
            let discr = match data.variants.len() {
                0..=255 => quote!{ emit_u8 },
                _ => quote!{ emit_usize },
            };
            
            let arms = data.variants.iter().enumerate().map(|(i, variant)| {
                let mut field_names = Vec::new();
                let fields = match &variant.fields {
                    syn::Fields::Named(fields) => {
                        let binders = fields.named.iter().enumerate().map(|(i, field)| {
                            let name = syn::Ident::new(&format!("_{}", i), field.ident.span());
                            let ident = &field.ident;
                            
                            field_names.push((name.clone(), &field.ty));
                            
                            quote!{ #ident: ref #name }
                        });
                        
                        quote!{ { #(#binders),* } }
                    },
                    syn::Fields::Unnamed(fields) => {
                        let binders = fields.unnamed.iter().enumerate().map(|(i, field)| {
                            let name = syn::Ident::new(&format!("_{}", i), field.ident.span());
                            
                            field_names.push((name.clone(), &field.ty));
                            
                            quote!{ ref #name }
                        });
                        
                        quote!{ ( #(#binders),* ) }
                    },
                    syn::Fields::Unit => quote!{},
                };
                
                let name = &variant.ident;
                let pat = quote!{ Self::#name #fields };
                let i = proc_macro2::Literal::usize_unsuffixed(i);
                let lines = field_names.into_iter().map(|(name, ty)| {
                    quote!{ <#ty as ::encode::Encode>::encode(#name, e)?; }
                });
                
                quote!{
                    #pat => {
                        e.#discr(#i)?;
                        #(#lines)*
                    }
                }
            });
            
            quote!{
                match self {
                    #(#arms),*
                }
            }
        },
        syn::Data::Union(_) => {
            quote!{
                let bytes: [u8; ::std::mem::size_of::<Self>()] = unsafe {
                    ::std::mem::transmute(self)
                };
                
                e.emit_bytes(&bytes[..])?;
            }
        },
    };

    let result = quote!{
        impl #generics ::encode::Encode for #type_name #generics where #where_clause {
            fn encode<ENCODER: ::encode::Encoder>(&self, e: &mut ENCODER) -> ::std::result::Result<(), ENCODER::Error> {
                #body
                
                ::std::result::Result::Ok(())
            }
        }
    };

    result.into()
}

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let datatype = syn::parse_macro_input!(input as syn::DeriveInput);
    let type_name = &datatype.ident;
    let generics = &datatype.generics;
    let mut where_clause = quote!{};
    let body = match datatype.data {
        syn::Data::Struct(data) => {
            for field in &data.fields {
                let ty = &field.ty;
                
                where_clause = quote!{ #where_clause #ty: ::encode::Decode, };
            }
            
            let mut field_names = Vec::new();
            
            let binders = data.fields.iter().enumerate().map(|(i, field)| {
                let name = syn::Ident::new(&format!("_{}", i), field.ident.span());
                let ty = &field.ty;
                
                field_names.push(name.clone());
                
                quote!{ let #name = <#ty as ::encode::Decode>::decode(d)?; }
            }).collect::<Vec<_>>();
            
            let cons = match &data.fields {
                syn::Fields::Named(fields) => {
                    let fields = fields.named.iter().zip(field_names.iter()).map(|(field, name)| {
                        let ident = &field.ident;
                        
                        quote!{ #ident: #name }
                    });
                    
                    quote!{ { #(#fields),* } }
                },
                syn::Fields::Unnamed(_) => {
                    quote!{ ( #(#field_names),* ) }
                },
                syn::Fields::Unit => quote!{},
            };
            
            quote!{
                #(#binders)*
                ::std::result::Result::Ok(Self #cons)
            }
        },
        syn::Data::Enum(data) => {
            for variant in &data.variants {
                for field in &variant.fields {
                    let ty = &field.ty;
                    
                    where_clause = quote!{ #where_clause #ty: ::encode::Decode, };
                }
            }
            
            let discr = match data.variants.len() {
                0..=255 => quote!{ read_u8 },
                _ => quote!{ read_usize },
            };
            
            let pat = data.variants.iter().enumerate().map(|(i, variant)| {
                let i = proc_macro2::Literal::usize_unsuffixed(i);
                let name = &variant.ident;
                let mut field_names = Vec::new();
            
                let binders = variant.fields.iter().enumerate().map(|(i, field)| {
                    let name = syn::Ident::new(&format!("_{}", i), field.ident.span());
                    let ty = &field.ty;
                    
                    field_names.push(name.clone());
                    
                    quote!{ let #name = <#ty as ::encode::Decode>::decode(d)?; }
                }).collect::<Vec<_>>();
                
                let cons = match &variant.fields {
                    syn::Fields::Named(fields) => {
                        let fields = fields.named.iter().zip(field_names.iter()).map(|(field, name)| {
                            let ident = &field.ident;
                            
                            quote!{ #ident: #name }
                        });
                        
                        quote!{ { #(#fields),* } }
                    },
                    syn::Fields::Unnamed(_) => {
                        quote!{ ( #(#field_names),* ) }
                    },
                    syn::Fields::Unit => quote!{},
                };
                
                quote!{
                    #i => {
                        #(#binders)*
                        ::std::result::Result::Ok(Self::#name #cons)
                    }
                }
            });
            
            quote!{
                match d.#discr()? {
                    #(#pat,)*
                    _ => ::std::result::Result::Err(d.error("invalid discriminant"))
                }
            }
        },
        syn::Data::Union(_) => {
            quote!{
                let bytes = d.read_bytes(::std::mem::size_of::<Self>())?;
                let ptr = bytes.as_ptr() as *const Self;
                
                Ok(unsafe { ::std::ptr::read(ptr) })
            }
        },
    };
    
    let result = quote!{
        impl #generics ::encode::Decode for #type_name #generics where #where_clause {
            fn decode<DECODER: ::encode::Decoder>(d: &mut DECODER) -> std::result::Result<Self, DECODER::Error> {
                #body
            }
        }
    };
    
    result.into()
}
