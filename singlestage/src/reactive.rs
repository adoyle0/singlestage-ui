use leptos::prelude::*;
use std::fmt::Debug;

#[cfg(feature = "stores")]
use reactive_stores::{ArcField, ArcStore, Field, Store, StoreField, Subfield};

// Rust gets upset about trying to use `#[cfg(feature = "stores")]` in a where clause:
// https://github.com/rust-lang/rust/issues/115590
// HACK: So we'll just define `Reactive` conditionally for now because it's easy and it works.

/// A reactive binding wrapper that can take any value and upgrade it to a RwSignal (default)
/// or a reactive_stores::Field (Store, ArcStore, Field, ArcField and Subfield can be converted).
///
/// For example:
/// ```rs
/// <Checkbox checked=true />
/// ```
/// is effectively the same as:
/// ```rs
/// <Checkbox checked=RwSignal::new(true) />
/// ```
/// A RwSignal defined elsewhere can also be used:
/// ```rs
/// let checked = RwSignal::new(true);
///
/// view!{
///     <Checkbox checked />
/// }
/// ```
/// In this case the `RwSignal` and the `Checkbox` are coupled. Changing one will update the other and notify all listeners.
#[cfg(feature = "stores")]
#[derive(Clone, Debug)]
pub enum Reactive<T>
where
    T: Send + Sync + Clone + 'static,
    RwSignal<T>: Send + Sync + Clone + Copy + 'static,
    Field<T>: Send + Sync + Clone + Copy + 'static,
{
    RwSignal(RwSignal<T>),
    Field(Field<T>),
}

/// A reactive binding wrapper that can take any value and upgrade it to a RwSignal (default)
/// or a reactive_stores::Field (Store, ArcStore, Field, ArcField and Subfield can be converted).
///
/// For example:
/// ```rs
/// <Checkbox checked=true />
/// ```
/// is effectively the same as:
/// ```rs
/// <Checkbox checked=RwSignal::new(true) />
/// ```
/// A RwSignal defined elsewhere can also be used:
/// ```rs
/// let checked = RwSignal::new(true);
///
/// view!{
///     <Checkbox checked />
/// }
/// ```
/// In this case the `RwSignal` and the `Checkbox` are coupled. Changing one will update the other and notify all listeners.
#[cfg(not(feature = "stores"))]
#[derive(Clone, Debug)]
pub enum Reactive<T>
where
    T: Send + Sync + Clone + 'static,
    RwSignal<T>: Send + Sync + Clone + Copy + 'static,
{
    RwSignal(RwSignal<T>),
}

impl<T> Reactive<T>
where
    T: Send + Sync + Clone + 'static,
{
    #[inline]
    pub fn get(&self) -> T {
        match self {
            Self::RwSignal(rw_signal) => rw_signal.get(),
            #[cfg(feature = "stores")]
            Self::Field(field) => field.get(),
        }
    }

    #[inline]
    pub fn get_untracked(&self) -> T {
        match self {
            Self::RwSignal(rw_signal) => rw_signal.get_untracked(),
            #[cfg(feature = "stores")]
            Self::Field(field) => field.get_untracked(),
        }
    }

    #[inline]
    pub fn set(&self, value: T) {
        match self {
            Self::RwSignal(rw_signal) => rw_signal.set(value),
            #[cfg(feature = "stores")]
            Self::Field(field) => field.set(value),
        }
    }

    #[inline]
    pub fn with<K>(&self, fun: impl FnOnce(&T) -> K) -> K {
        match self {
            Self::RwSignal(rw_signal) => rw_signal.with(fun),
            #[cfg(feature = "stores")]
            Self::Field(field) => field.with(fun),
        }
    }

    #[inline]
    pub fn update(&self, fun: impl FnOnce(&mut T)) {
        match self {
            Self::RwSignal(rw_signal) => rw_signal.update(fun),
            #[cfg(feature = "stores")]
            Self::Field(field) => field.update(fun),
        }
    }
}

impl<T: Send + Sync + Clone> Copy for Reactive<T> {}

impl<T: Default + Send + Clone + Sync + 'static> Default for Reactive<T> {
    fn default() -> Self {
        Self::RwSignal(RwSignal::<T>::new(Default::default()))
    }
}

impl<T: Default + Send + Clone + Sync + 'static> Reactive<T> {
    pub fn new(value: T) -> Self {
        Self::RwSignal(RwSignal::<T>::new(value))
    }
}

impl From<&str> for Reactive<String> {
    fn from(value: &str) -> Self {
        Reactive::RwSignal(RwSignal::new(value.to_string()))
    }
}

impl<T> From<T> for Reactive<T>
where
    T: Send + Sync + Clone + 'static,
{
    fn from(value: T) -> Self {
        Reactive::RwSignal(RwSignal::new(value))
    }
}

impl<T> From<RwSignal<T>> for Reactive<T>
where
    T: Send + Sync + Clone + 'static,
{
    fn from(value: RwSignal<T>) -> Self {
        Reactive::RwSignal(value)
    }
}

#[cfg(feature = "stores")]
impl<Inner, Prev, T> From<Subfield<Inner, Prev, T>> for Reactive<T>
where
    Inner: Send + Sync + StoreField<Value = Prev> + 'static,
    Prev: 'static,
    T: Send + Sync + Clone + 'static,
    Subfield<Inner, Prev, T>: Get<Value = T> + Set<Value = T> + Clone + Track,
{
    fn from(value: Subfield<Inner, Prev, T>) -> Self {
        Reactive::Field(value.into())
    }
}

#[cfg(feature = "stores")]
impl<T> From<Field<T>> for Reactive<T>
where
    T: Send + Sync + Clone + 'static,
    Field<T>: Get<Value = T> + Set<Value = T> + Clone + Track,
{
    fn from(value: Field<T>) -> Self {
        Reactive::Field(value)
    }
}

#[cfg(feature = "stores")]
impl<T> From<Store<T>> for Reactive<T>
where
    T: Send + Sync + Clone + 'static,
    Store<T>: Get<Value = T> + Set<Value = T> + Clone + Track,
{
    fn from(value: Store<T>) -> Self {
        Reactive::Field(value.into())
    }
}

#[cfg(feature = "stores")]
impl<T> From<ArcField<T>> for Reactive<T>
where
    T: Send + Sync + Clone + 'static,
    ArcField<T>: Get<Value = T> + Set<Value = T> + Clone + Track,
{
    fn from(value: ArcField<T>) -> Self {
        Reactive::Field(value.into())
    }
}

#[cfg(feature = "stores")]
impl<T> From<ArcStore<T>> for Reactive<T>
where
    T: Send + Sync + Clone + 'static,
    ArcStore<T>: Get<Value = T> + Set<Value = T> + Clone + Track,
{
    fn from(value: ArcStore<T>) -> Self {
        Reactive::Field(value.into())
    }
}
