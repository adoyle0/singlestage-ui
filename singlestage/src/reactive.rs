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
    Field<T>: Send + Sync + Clone + 'static,
    RwSignal<T>: Send + Sync + Clone + 'static,
{
    Field(Field<T>),
    RwSignal(RwSignal<T>),
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
    RwSignal<T>: Send + Sync + Clone + 'static,
{
    RwSignal(RwSignal<T>),
}

impl<T> Reactive<T>
where
    T: Send + Sync + Clone + 'static,
{
    pub fn get(&self) -> T {
        match self {
            Self::RwSignal(signal) => signal.get(),
            #[cfg(feature = "stores")]
            Self::Field(field) => field.get(),
        }
    }

    pub fn get_untracked(&self) -> T {
        match self {
            Self::RwSignal(signal) => signal.get_untracked(),
            #[cfg(feature = "stores")]
            Self::Field(field) => field.get_untracked(),
        }
    }

    pub fn set(&self, value: T) {
        match self {
            Self::RwSignal(signal) => signal.set(value),
            #[cfg(feature = "stores")]
            Self::Field(field) => field.set(value),
        }
    }

    pub fn with<K>(&self, fun: impl FnOnce(&T) -> K) -> K {
        match self {
            Self::RwSignal(signal) => signal.with(fun),
            #[cfg(feature = "stores")]
            Self::Field(field) => field.with(fun),
        }
    }

    pub fn update(&self, fun: impl FnOnce(&mut T)) {
        match self {
            Self::RwSignal(signal) => signal.update(fun),
            #[cfg(feature = "stores")]
            Self::Field(field) => field.update(fun),
        }
    }
}

impl<T> Copy for Reactive<T> where T: Send + Clone + Sync {}

impl<T> Default for Reactive<T>
where
    T: Default + Send + Clone + Sync + 'static,
{
    fn default() -> Self {
        Self::RwSignal(RwSignal::<T>::new(Default::default()))
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

// impl<T> From<impl FnOnce(&T)> for Reactive<T>
// where
//     T: Send + Sync + Clone + 'static,
// {
//     fn from(fun: impl FnOnce(&T)) {
//         Reactive::RwSignal(RwSignal::new(fun))
//     }
// }

impl<T> From<T> for Reactive<Vec<T>>
where
    T: Send + Sync + Clone + 'static,
{
    fn from(value: T) -> Self {
        Reactive::RwSignal(RwSignal::new(vec![value]))
    }
}

impl<T> From<Option<T>> for Reactive<T>
where
    T: Send + Sync + Clone + 'static + Default,
{
    fn from(value: Option<T>) -> Self {
        Reactive::RwSignal(RwSignal::new(value.unwrap_or_default()))
    }
}

impl<T> From<Option<T>> for Reactive<Vec<T>>
where
    T: Send + Sync + Clone + 'static + Default,
{
    fn from(value: Option<T>) -> Self {
        Reactive::RwSignal(RwSignal::new(vec![value.unwrap_or_default()]))
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

impl<T> From<MaybeProp<T>> for Reactive<T>
where
    T: Send + Sync + Clone + 'static + Default,
{
    fn from(value: MaybeProp<T>) -> Self {
        Reactive::RwSignal(RwSignal::new(value.get_untracked().unwrap_or_default()))
    }
}

impl<T> From<ReadSignal<T>> for Reactive<T>
where
    T: Send + Sync + Clone + 'static,
{
    fn from(value: ReadSignal<T>) -> Self {
        Reactive::RwSignal(RwSignal::new(value.get_untracked()))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reactive_from_t() {
        let reactive: Reactive<usize> = 1.into();
        assert_eq!(reactive.get_untracked(), 1);

        reactive.set(2);
        assert_eq!(reactive.get_untracked(), 2);
    }

    #[test]
    fn reactive_from_vec_t() {
        let reactive: Reactive<Vec<usize>> = 1.into();
        assert_eq!(reactive.get_untracked(), vec![1]);

        reactive.set(vec![2]);
        assert_eq!(reactive.get_untracked(), vec![2]);
    }

    #[test]
    fn reactive_from_rwsignal_t() {
        let reactive: Reactive<usize> = RwSignal::new(1).into();
        assert_eq!(reactive.get_untracked(), 1);

        reactive.set(2);
        assert_eq!(reactive.get_untracked(), 2);
    }

    #[test]
    fn reactive_from_maybeprop_t() {
        let maybe_prop: MaybeProp<usize> = 1.into();
        let reactive: Reactive<usize> = Reactive::from(maybe_prop);
        assert_eq!(reactive.get_untracked(), 1);

        reactive.set(2);
        assert_eq!(reactive.get_untracked(), 2);
    }

    #[test]
    fn reactive_set() {
        let reactive: Reactive<usize> = Reactive::from(1);
        assert_eq!(reactive.get_untracked(), 1);

        reactive.set(2);
        assert_eq!(reactive.get_untracked(), 2);
    }

    #[test]
    fn reactive_update() {
        let reactive: Reactive<usize> = Reactive::from(1);
        assert_eq!(reactive.get_untracked(), 1);

        reactive.update(|val| *val += 1);
        assert_eq!(reactive.get_untracked(), 2);
    }

    // TODO: Test stores
}
