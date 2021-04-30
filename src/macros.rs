#[macro_export]
macro_rules! impl_into_iter {
    ($container:ty, $slice:ty, $field:ident) => {
        impl IntoIterator for $container {
            type Item = $slice;
            type IntoIter = std::vec::IntoIter<$slice>;
            fn into_iter(self) -> Self::IntoIter {
                self.$field.into_iter()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_iter_and_mut {
    ($container:ty, $slice:ty, $field:ident) => {
        impl $container {
            pub fn iter<'iter>(&'iter self) -> core::slice::Iter<'iter, $slice> {
                self.$field.iter()
            }

            pub fn iter_mut<'iter_mut>(
                &'iter_mut mut self,
            ) -> core::slice::IterMut<'iter_mut, $slice> {
                self.$field.iter_mut()
            }
        }
    };
}
