extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn;

fn enum_liter_by_pos(pos:&usize, len:&usize) -> proc_macro2::Literal {
    match *len {
        n if n <= u8::MAX as usize => { proc_macro2::Literal::u8_suffixed(*pos as u8) }
        n if n <= u16::MAX as usize => { proc_macro2::Literal::u16_suffixed(*pos as u16) }
        _ => { proc_macro2::Literal::usize_suffixed(*pos) }
    }
}

fn enum_ident_by_variants_len(len:&usize) -> proc_macro2::Ident {
    match *len {
        n if n <= u8::MAX as usize => { proc_macro2::Ident::new("u8", proc_macro2::Span::call_site()) }
        n if n <= u16::MAX as usize => { proc_macro2::Ident::new("u16", proc_macro2::Span::call_site()) }
        _ => { proc_macro2::Ident::new("usize", proc_macro2::Span::call_site()) }
    }
}

fn writer_by_field_ty(f:&syn::Field, is_enum: bool) -> TokenStream2 {
    match &f.ty {
        syn::Type::Path(tp) if f.ident.is_some() && !is_enum  => {
            if tp.path.segments.len() == 1 {
                match &tp.path.segments.first().unwrap().ident.to_string()[..] {
                    "String" | "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" | "usize" |
                    "f64" | "f32" | "bool" | "()" => {
                        let field_name = f.ident.as_ref().unwrap();
                        return quote!(
                            self.#field_name.proto_write(buf);
                        );
                    }
                    v => {
                        unimplemented!("for {} {}", v, f.ident.as_ref().unwrap().to_string())
                    }
                }
            } else { unimplemented!("for none ident") }
        }

        syn::Type::Path(tp) if is_enum => {
            if tp.path.segments.len() == 1 {
                let first_sement = tp.path.segments.first().unwrap();
                let pidend = &first_sement.ident;
                match &pidend.to_string()[..] {
                    "String" | "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" | "usize" |
                    "f64" | "f32" | "bool" | "()" => {
                        return quote!(
                            v.proto_write(buf);
                        );
                    }
                    v => {
                        unimplemented!("for {} {}", v, f.ident.as_ref().unwrap().to_string())
                    }
                }
            } else { unimplemented!("for none ident") }
        }
    
        _ => unimplemented!("for {:?}", f)
    }
}

fn impl_proto_writer(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let mut writers = quote!();
    
    let gen = 
        match &ast.data {
            syn::Data::Struct(s) => {   
                if s.fields.is_empty() {
                    unimplemented!("for empty structs")
                }

                for f in s.fields.iter() {
                    let writer = writer_by_field_ty(f, false);
                    writers.extend(quote!(#writer));
                }                    

                quote! {
                    impl proto_buffer::ProtoWriter for #name {
                        fn proto_write(&self, buf:&mut proto_buffer::Buffer) {
                            #writers
                        }
                    }
                }
            }

            syn::Data::Enum(syn::DataEnum {variants, ..}) => { 
                if variants.is_empty() {
                    unimplemented!("for empty enums")
                }

                let eliter_ty = enum_ident_by_variants_len(&variants.len());

                for (pos, v) in variants.iter().enumerate() {                    
                    let enum_name = &v.ident;

                    let eliter = enum_liter_by_pos(&pos, &variants.len());

                    match v.fields.len() {
                        0 => {
                            writers.extend(quote!(
                                #name::#enum_name => {
                                    #eliter_ty::proto_write(&#eliter, buf) 
                                }
                            ));
                        }
                        1 => {
                            let f = &v.fields.iter().next().unwrap();
                            let writer = writer_by_field_ty(f, true);

                            writers.extend(quote!(
                                #name::#enum_name(v) => {
                                    #eliter_ty::proto_write(&#eliter, buf);
                                    #writer 
                                }
                            ));
                        }
                        n => {
                            unimplemented!("for {} fields in enum", n)
                        }
                    }
                }

                quote! {
                    impl proto_buffer::ProtoWriter for #name {
                        fn proto_write(&self, buf:&mut proto_buffer::Buffer) {
                            match self {
                                #writers
                            }
                        }
                    }
                }
            }

            syn::Data::Union(_) => { unimplemented!("for Union") }
        };

    //println!("{}", gen);
    gen.into()
}

fn reader_by_field_ty(f:&syn::Field, is_enum: bool) -> TokenStream2 {
    match &f.ty {
        syn::Type::Path(tp) if f.ident.is_some() && !is_enum  => {
            if tp.path.segments.len() == 1 {
                match &tp.path.segments.first().unwrap().ident.to_string()[..] {
                    "String" | "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" | "usize" |
                    "f64" | "f32" | "bool" | "()" => {
                        let field_name = f.ident.as_ref().unwrap();
                        let ty = &tp.path.segments.first().unwrap().ident;
                        return quote!(
                            #field_name: #ty::proto_read(buf),
                        );
                    }
                    v => {
                        unimplemented!("for {} {}", v, f.ident.as_ref().unwrap().to_string())
                    }
                }
            } else { unimplemented!("for none ident") }
        }

        syn::Type::Path(tp) if f.ident.is_none() && is_enum  => {
            if tp.path.segments.len() == 1 {
                match &tp.path.segments.first().unwrap().ident.to_string()[..] {
                    "String" | "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" | "usize" |
                    "f64" | "f32" | "bool" | "()" => {
                        let ty = &tp.path.segments.first().unwrap().ident;
                        return quote!(
                            #ty::proto_read(buf)
                        );
                    }
                    v => {
                        unimplemented!("for {} {}", v, f.ident.as_ref().unwrap().to_string())
                    }
                }
            } else { unimplemented!("for none ident") }
        }
    
        _ => unimplemented!("for {:?}", f)
    }    
}

fn impl_proto_reader(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let mut readers = quote!();
    
    let gen = 
        match &ast.data {
            syn::Data::Struct(s) => {   
                for f in s.fields.iter() {
                    let reader = reader_by_field_ty(&f, false);
                    readers.extend(quote!(#reader));
                }                    

                quote! {
                    impl proto_buffer::ProtoReader for #name {
                        fn proto_read(buf:&mut proto_buffer::Buffer) -> Self {
                           return #name {
                                #readers
                           }
                        }
                    }
                }
            }
            syn::Data::Enum(syn::DataEnum {variants, ..}) => { 
                if variants.is_empty() {
                    unimplemented!("for empty enums")
                }

                let eliter_ty = enum_ident_by_variants_len(&variants.len());

                for (pos, v) in variants.iter().enumerate() {                    
                    let eliter = enum_liter_by_pos(&pos, &variants.len());

                    let enum_name = &v.ident;

                    match v.fields.len() {
                        0 => {
                            readers.extend(quote!(
                                #eliter => { #name::#enum_name }
                            ));
                        }
                        1 => {
                            let f = &v.fields.iter().next().unwrap();
                            let reader = reader_by_field_ty(f, true);

                            readers.extend(quote!(
                                #eliter => {
                                    #name::#enum_name(#reader)
                                }
                            ));
                        }
                        n => {
                            unimplemented!("for {} fields in enum", n)
                        }
                    }
                }

                let name_str = name.to_string();

                quote! {
                    impl proto_buffer::ProtoReader for #name {
                        fn proto_read(buf:&mut proto_buffer::Buffer) -> Self {
                           return match #eliter_ty::proto_read(buf) {
                                #readers
                                n => panic!("wrong read {} from {}", #name_str, n)
                           }
                        }
                    }
                }
            }
            syn::Data::Union(_) => { unimplemented!("for Union") }
        };

    //println!("{}", gen);
    gen.into()
}

#[proc_macro_derive(ProtoBufferReader)]
pub fn proto_buffer_reader_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_proto_reader(&ast)
}

#[proc_macro_derive(ProtoBufferWriter)]
pub fn proto_buffer_writer_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_proto_writer(&ast)
}


#[cfg(test)]
mod tests {
}
