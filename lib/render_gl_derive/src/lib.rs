#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

#[proc_macro_derive(VertexAttribPointers, attributes(location))]
pub fn vertex_attrib_pointers_derive(
   input: proc_macro::TokenStream
) -> proc_macro::TokenStream {
   //  构造类型定义的字符串表示
   let s = input.to_string();

   // Parse the string representation
   let ast = syn::parse_derive_input(&s).unwrap();

   // code-gen
   let gen = generate_impl(&ast);

   // Return the generated impl
   gen.parse().unwrap()
}

fn generate_impl(
   ast: &syn::DeriveInput
) -> quote::Tokens {
   let ident = &ast.ident;
   let generics = &ast.generics;
   let where_clause = &ast.generics.where_clause;
   let fields_vertex_attrib_pointer = generate_vertex_attrib_pointer_calls(&ast.body);

   quote! {
      impl #ident #generics #where_clause {
         #[allow(unused_variables)]
         pub fn vertex_attrib_pointers(gl: &::gl::Gl) {
            let stride = ::std::mem::size_of::<Self>();
            let offset = 0;
            #(#fields_vertex_attrib_pointer)*
         }
      }
   }
}
fn generate_vertex_attrib_pointer_calls(body: &syn::Body) -> Vec<quote::Tokens> {
   match body {
      &syn::Body::Enum(_) => panic!("VertexAttribPointers无法被声明为枚举"),
      &syn::Body::Struct(syn::VariantData::Unit) => {
         panic!("VertexAttribPointers 无法被声明为 Unit structs")
      }
      &syn::Body::Struct(syn::VariantData::Tuple(_)) => {
         panic!("VertexAttribPointers 无法被声明为 Tuple structs")
      }
      &syn::Body::Struct(syn::VariantData::Struct(ref s)) => s
              .iter()
              .map(generate_struct_field_vertex_attrib_pointer_call)
              .collect(),
   }
}

fn generate_struct_field_vertex_attrib_pointer_call(field: &syn::Field) -> quote::Tokens {
   let field_name = match field.ident {
      Some(ref i) => format!("{}", i),
      None => String::from(""),
   };
   let location_attr = field
           .attrs
           .iter()
           .filter(|a| a.value.name() == "location")
           .next()
           .unwrap_or_else(|| panic!("字段 {} 缺失 #[location = ?] 的附加属性", field_name));

   let location_value: usize = match location_attr.value {
      syn::MetaItem::NameValue(_, syn::Lit::Str(ref s, _)) => s.parse().unwrap_or_else(|_| {
         panic!(
            "字段-{} location 附加属性的取值必须是整形或字符串字面常量",field_name
         )
      }),
      _ => panic!(
         "字段-{} location 附加属性的取值必须是整形或字符串字面常量",field_name
      ),
   };

   let field_ty = &field.ty;
   quote! {
        let location = #location_value;
        unsafe {
            #field_ty::vertex_attrib_pointer(gl, stride, location, offset);
        }
        let offset = offset + ::std::mem::size_of::<#field_ty>();
    }
}