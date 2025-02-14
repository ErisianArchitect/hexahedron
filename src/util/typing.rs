
pub struct IsZst<T>(std::marker::PhantomData<T>);

impl<T> IsZst<T> {
    pub const VALUE: bool = std::mem::size_of::<T>() == 0;
}

#[inline(always)]
pub const fn is_zst<T>() -> bool {
    IsZst::<T>::VALUE
}