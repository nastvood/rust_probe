extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

fn impl_proto_writer(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let mut writers = quote!();
    
    let gen = 
        match &ast.data {
            syn::Data::Struct(s) => {   
                for f in s.fields.iter() {
                    match &f.ty {
                        syn::Type::Path(tp) if f.ident.is_some()  => {
                            if tp.path.segments.len() == 1 {
                                match &tp.path.segments.first().unwrap().ident.to_string()[..] {
                                    "String" | "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" | "usize" | 
                                    "f64" | "f32" | "bool" | "()" => {
                                        let field_name = f.ident.as_ref().unwrap();
                                        writers.extend(quote!(
                                            self.#field_name.proto_write(buf);
                                        ));
                                    }
                                    v => {
                                        unimplemented!("for {} {}", v, f.ident.as_ref().unwrap().to_string())
                                    }
                                }
                            } else { unimplemented!("for none ident") }                                
                        }

                        _ => unimplemented!("for {:?}", s)
                    }
                }                    

                quote! {
                    impl proto_buffer::ProtoWriter for #name {
                        fn proto_write(&self, buf:&mut proto_buffer::Buffer) {
                            #writers
                        }
                    }
                }
            }
            syn::Data::Enum(_) => { unimplemented!("for enum") }
            syn::Data::Union(_) => { unimplemented!("for Union") }
        };

    gen.into()
}

fn impl_proto_reader(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let mut readers = quote!();
    
    let gen = 
        match &ast.data {
            syn::Data::Struct(s) => {   
                for f in s.fields.iter() {
                    match &f.ty {
                        syn::Type::Path(tp) if f.ident.is_some()  => {
                            if tp.path.segments.len() == 1 {
                                match &tp.path.segments.first().unwrap().ident.to_string()[..] {
                                    "String" | "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" | "usize" | 
                                    "f64" | "f32" | "bool" | "()" => {
                                        let field_name = f.ident.as_ref().unwrap();
                                        let ty = &tp.path.segments.first().unwrap().ident;
                                        readers.extend(quote!(
                                            #field_name: #ty::proto_read(buf),
                                        ));
                                    }
                                    v => {
                                        unimplemented!("for {} {}", v, f.ident.as_ref().unwrap().to_string())
                                    }
                                }
                            } else { unimplemented!("for none ident") }                                
                        }

                        _ => unimplemented!("for {:?}", s)
                    }
                }                    

                quote! {
                    impl proto_buffer::ProtoReader for #name {
                        fn proto_read(buf:&mut proto_buffer::Buffer) -> Self {
                           return #name {
                                #readers
                               /*name: String::proto_read(buf),
                               email: String::proto_read(buf),
                               age: u8::proto_read(buf)*/
                           }
                        }
                    }
                }
            }
            syn::Data::Enum(_) => { unimplemented!("for enum") }
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
