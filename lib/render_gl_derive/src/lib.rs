extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(VertexAttribPointers, attributes(location))]
pub fn vertex_attrib_pointers_derive(
   input: TokenStream
) -> TokenStream {
   // Parse the input tokens into a syntax tree
   let ast = parse_macro_input!(input as DeriveInput);

   let ident = &ast.ident;
   let generics = &ast.generics;
   let where_clause = &ast.generics.where_clause;

   let fields_vertex_attrib_pointer =
      generate_vertex_attrib_pointer_calls(&ast.data);

   // Build the output,using quasi-quotation
   let expanded: proc_macro2::TokenStream = quote! {
      impl #ident #generics #where_clause {
         #[allow(unused_variables)]
         pub fn vertex_attrib_pointers() {
            let stride = std::mem::size_of::<Self>();
            let offset = 0;
            #(#fields_vertex_attrib_pointer)*
         }
      }
   }.into();

   // Hand the output tokens back to the compiler
   TokenStream::from(expanded)
}


fn generate_vertex_attrib_pointer_calls(
   body: &syn::Data
) -> Vec<proc_macro2::TokenStream> {
   match body {
      &syn::Data::Enum(_) => panic!("VertexAttribPointers无法被声明为枚举"),
      &syn::Data::Union(_) => panic!("VertexAttribPointers 无法被声明为 Tuple structs"),

      &syn::Data::Struct(ref s) => s.fields
         .iter()
         .map(generate_struct_field_vertex_attrib_pointer_call)
         .collect()
   }
}
fn generate_struct_field_vertex_attrib_pointer_call(
   field: &syn::Field
) -> proc_macro2::TokenStream {
   let field_name = match field.ident {
      Some(ref i) => format!("{}", i),
      None => String::from(""),
   };
   let location_attr = field
           .attrs
           .iter()
           .filter(|a| a.path.get_ident().unwrap().to_string() == "location")
           .next()
           .unwrap_or_else(|| panic!("字段 {} 缺失 #[location = ?] 的附加属性", field_name));

   let location_value: usize = match location_attr.parse_meta().unwrap_or_else(
      |_| panic!("Failed to parse meta location")
   ) {
      syn::Meta::NameValue(value) => {
         match value.lit {
            syn::Lit::Int(i) => i.to_string().parse::<usize>().unwrap(),
            _ => panic!("location值的类型尚未支持")
         }
      },
      _ => {
         panic!("location值的类型尚未支持")
      }
   };

   let field_ty = &field.ty;
   quote! {
      let location = #location_value;
      unsafe {
         #field_ty::vertex_attrib_pointer(stride, location, offset);
      }
      let offset = offset + std::mem::size_of::<#field_ty>();
   }
}