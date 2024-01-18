mod widget_builder;

use crate::widget_builder::derive_widget_builder_impl;

use proc_macro::TokenStream;

#[proc_macro_derive(WidgetBuilder, attributes(widget_builder))]
pub fn derive_widget_builder(input_item: TokenStream) -> TokenStream {
	TokenStream::from(derive_widget_builder_impl(input_item.into()).unwrap_or_else(|err| err.to_compile_error()))
}