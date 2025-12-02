

#[macro_export]
macro_rules! define_getters {
    (
        $struct_name:ident,
        $ret:ty,
        $( $fn_name:ident, $provider:ident, $query:ident :: $variant:ident );* $(;)?
    ) => {
        impl<'a> $struct_name<'a> {
            $(
                pub async fn $fn_name(&self) -> $crate::version::Result<$ret> {
                    $provider.get(&self, $query::$variant).await
                }
            )*
        }
    };
}



