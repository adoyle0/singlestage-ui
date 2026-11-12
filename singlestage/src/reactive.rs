// TODO: Test stores
// TODO: Test 'static requirement for stores
// TODO: From Memo/ReadSignal reactivity?

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
    RwSignal<T>: Send + Sync + Clone,
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
    RwSignal<T>: Send + Sync + Clone,
{
    RwSignal(RwSignal<T>),
}

impl<T> Reactive<T>
where
    T: Send + Sync + Clone,
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

    pub fn update(&self, fun: impl FnOnce(&mut T)) {
        match self {
            Self::RwSignal(signal) => signal.update(fun),
            #[cfg(feature = "stores")]
            Self::Field(field) => field.update(fun),
        }
    }
}

impl<T> Copy for Reactive<T> where T: Send + Sync + Clone {}

impl<T> Default for Reactive<T>
where
    T: Default + Send + Sync + Clone,
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
    T: Send + Sync + Clone,
{
    fn from(value: T) -> Self {
        Reactive::RwSignal(RwSignal::new(value))
    }
}

impl<T> From<T> for Reactive<Vec<T>>
where
    T: Send + Sync + Clone,
{
    fn from(value: T) -> Self {
        Reactive::RwSignal(RwSignal::new(vec![value]))
    }
}

impl<T> From<Option<T>> for Reactive<T>
where
    T: Send + Sync + Clone + Default,
{
    fn from(value: Option<T>) -> Self {
        Reactive::RwSignal(RwSignal::new(value.unwrap_or_default()))
    }
}

impl<T> From<Option<T>> for Reactive<Vec<T>>
where
    T: Send + Sync + Clone + Default,
{
    fn from(value: Option<T>) -> Self {
        Reactive::RwSignal(RwSignal::new(vec![value.unwrap_or_default()]))
    }
}

impl<T> From<RwSignal<T>> for Reactive<T>
where
    T: Send + Sync + Clone,
{
    fn from(value: RwSignal<T>) -> Self {
        Reactive::RwSignal(value)
    }
}

impl<T> From<MaybeProp<T>> for Reactive<T>
where
    T: Send + Sync + Clone + Default,
{
    fn from(value: MaybeProp<T>) -> Self {
        Reactive::RwSignal(RwSignal::new(value.get_untracked().unwrap_or_default()))
    }
}

impl<T> From<MaybeProp<Option<T>>> for Reactive<T>
where
    T: Send + Sync + Clone + Default,
{
    fn from(value: MaybeProp<Option<T>>) -> Self {
        Reactive::RwSignal(RwSignal::new(
            value
                .get_untracked()
                .unwrap_or_default()
                .unwrap_or_default(),
        ))
    }
}

impl<T> From<ReadSignal<T>> for Reactive<T>
where
    T: Send + Sync + Clone,
{
    fn from(value: ReadSignal<T>) -> Self {
        Reactive::RwSignal(RwSignal::new(value.get_untracked()))
    }
}

impl<T> From<(ReadSignal<T>, WriteSignal<T>)> for Reactive<T>
where
    T: Send + Sync + Clone + Default,
{
    fn from(value: (ReadSignal<T>, WriteSignal<T>)) -> Self {
        Reactive::RwSignal(
            RwSignal::unite(value.0, value.1).unwrap_or(RwSignal::new(<T>::default())),
        )
    }
}

impl<T> From<Memo<T>> for Reactive<T>
where
    T: Send + Sync + Clone,
{
    fn from(value: Memo<T>) -> Self {
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
mod test {
    use super::*;

    // T
    #[test]
    fn from_t_get() {
        let reactive: Reactive<usize> = 1.into();
        assert_eq!(reactive.get(), 1);
    }

    #[test]
    fn from_t_get_untracked() {
        let reactive: Reactive<usize> = 1.into();
        assert_eq!(reactive.get_untracked(), 1);
    }

    #[test]
    fn from_t_set() {
        let reactive: Reactive<usize> = 1.into();
        reactive.set(2);
        assert_eq!(reactive.get_untracked(), 2);
    }

    #[test]
    fn from_t_update() {
        let reactive: Reactive<usize> = 1.into();
        reactive.update(|v| *v += 1);
        assert_eq!(reactive.get_untracked(), 2);
    }

    // Option<T>
    #[test]
    fn from_option_t_get() {
        let reactive: Reactive<usize> = Some(1).into();
        assert_eq!(reactive.get(), 1);
    }

    #[test]
    fn from_option_t_get_untracked() {
        let reactive: Reactive<usize> = Some(1).into();
        assert_eq!(reactive.get_untracked(), 1);
    }

    #[test]
    fn from_option_t_set() {
        let reactive: Reactive<usize> = Some(1).into();
        reactive.set(2);
        assert_eq!(reactive.get_untracked(), 2);
    }

    #[test]
    fn from_option_t_update() {
        let reactive: Reactive<usize> = Some(1).into();
        reactive.update(|v| *v += 1);
        assert_eq!(reactive.get_untracked(), 2);
    }

    // Vec<T>
    #[test]
    fn from_vec_t_get() {
        let reactive: Reactive<Vec<usize>> = 1.into();
        assert_eq!(reactive.get(), vec![1]);
    }

    #[test]
    fn from_vec_t_get_untracked() {
        let reactive: Reactive<Vec<usize>> = 1.into();
        assert_eq!(reactive.get_untracked(), vec![1]);
    }

    #[test]
    fn from_vec_t_set() {
        let reactive: Reactive<Vec<usize>> = 1.into();
        reactive.set(vec![2]);
        assert_eq!(reactive.get_untracked(), vec![2]);
    }

    #[test]
    fn from_vec_t_update() {
        let reactive: Reactive<Vec<usize>> = 1.into();
        reactive.update(|v| v[0] += 1);
        assert_eq!(reactive.get_untracked(), vec![2]);
    }

    // Vec<Option<T>>
    #[test]
    fn from_vec_option_t_get() {
        let reactive: Reactive<Vec<usize>> = Some(1).into();
        assert_eq!(reactive.get(), vec![1]);
    }

    #[test]
    fn from_vec_option_t_get_untracked() {
        let reactive: Reactive<Vec<usize>> = Some(1).into();
        assert_eq!(reactive.get_untracked(), vec![1]);
    }

    #[test]
    fn from_vec_option_t_set() {
        let reactive: Reactive<Vec<usize>> = Some(1).into();
        reactive.set(vec![2]);
        assert_eq!(reactive.get_untracked(), vec![2]);
    }

    #[test]
    fn from_vec_option_t_update() {
        let reactive: Reactive<Vec<usize>> = Some(1).into();
        reactive.update(|v| v[0] += 1);
        assert_eq!(reactive.get_untracked(), vec![2]);
    }

    // MaybeProp<T>
    #[test]
    fn from_maybeprop_t_get() {
        let maybe_prop: MaybeProp<usize> = 1.into();
        let reactive: Reactive<usize> = Reactive::from(maybe_prop);
        assert_eq!(reactive.get(), 1);
    }

    #[test]
    fn from_maybeprop_t_get_untracked() {
        let maybe_prop: MaybeProp<usize> = 1.into();
        let reactive: Reactive<usize> = Reactive::from(maybe_prop);
        assert_eq!(reactive.get_untracked(), 1);
    }

    #[test]
    fn from_maybeprop_t_set() {
        let maybe_prop: MaybeProp<usize> = 1.into();
        let reactive: Reactive<usize> = Reactive::from(maybe_prop);
        reactive.set(2);
        assert_eq!(reactive.get_untracked(), 2);
    }

    #[test]
    fn from_maybeprop_t_update() {
        let maybe_prop: MaybeProp<usize> = 1.into();
        let reactive: Reactive<usize> = Reactive::from(maybe_prop);
        reactive.update(|v| *v += 1);
        assert_eq!(reactive.get_untracked(), 2);
    }

    // RwSignal<T>
    #[test]
    fn from_rwsignal_t_get() {
        let reactive: Reactive<usize> = RwSignal::new(1).into();
        assert_eq!(reactive.get(), 1);
    }

    #[test]
    fn from_rwsignal_t_get_untracked() {
        let reactive: Reactive<usize> = RwSignal::new(1).into();
        assert_eq!(reactive.get_untracked(), 1);
    }

    #[test]
    fn from_rwsignal_t_set() {
        let reactive: Reactive<usize> = RwSignal::new(1).into();
        reactive.set(2);
        assert_eq!(reactive.get_untracked(), 2);
    }

    #[test]
    fn from_rwsignal_t_update() {
        let reactive: Reactive<usize> = RwSignal::new(1).into();
        reactive.update(|v| *v += 1);
        assert_eq!(reactive.get_untracked(), 2);
    }

    #[test]
    fn from_rwsignal_t_reactive() {
        let rw_signal: RwSignal<usize> = RwSignal::new(1);
        let reactive: Reactive<usize> = rw_signal.into();
        rw_signal.set(2);
        assert_eq!(reactive.get_untracked(), 2);
    }

    // ReadSignal<T>
    #[test]
    fn from_readsignal_t_get() {
        let (read_signal, _write_signal): (ReadSignal<usize>, WriteSignal<usize>) = signal(1);
        let reactive: Reactive<usize> = Reactive::from(read_signal);
        assert_eq!(reactive.get(), 1);
    }

    #[test]
    fn from_readsignal_t_get_untracked() {
        let (read_signal, _write_signal): (ReadSignal<usize>, WriteSignal<usize>) = signal(1);
        let reactive: Reactive<usize> = Reactive::from(read_signal);
        assert_eq!(reactive.get_untracked(), 1);
    }

    #[test]
    fn from_readsignal_t_set() {
        let (read_signal, _write_signal): (ReadSignal<usize>, WriteSignal<usize>) = signal(1);
        let reactive: Reactive<usize> = Reactive::from(read_signal);
        reactive.set(2);
        assert_eq!(reactive.get_untracked(), 2);
    }

    #[test]
    fn from_readsignal_t_update() {
        let (read_signal, _write_signal): (ReadSignal<usize>, WriteSignal<usize>) = signal(1);
        let reactive: Reactive<usize> = Reactive::from(read_signal);
        reactive.update(|v| *v += 1);
        assert_eq!(reactive.get_untracked(), 2);
    }

    // TODO: ReadSignal reactivity
    // #[test]
    // fn from_readsignal_t_reactive() {
    //     let (read_signal, write_signal): (ReadSignal<usize>, WriteSignal<usize>) = signal(1);
    //     let reactive: Reactive<usize> = Reactive::from(read_signal);
    //     write_signal.set(2);
    //     assert_eq!(reactive.get_untracked(), 2);
    // }

    // (ReadSignal<T>, WriteSignal<T>)
    #[test]
    fn from_signal_t_get() {
        let (read_signal, write_signal): (ReadSignal<usize>, WriteSignal<usize>) = signal(1);
        let reactive: Reactive<usize> = Reactive::from((read_signal, write_signal));
        assert_eq!(reactive.get(), 1);
    }

    #[test]
    fn from_signal_t_get_untracked() {
        let (read_signal, write_signal): (ReadSignal<usize>, WriteSignal<usize>) = signal(1);
        let reactive: Reactive<usize> = Reactive::from((read_signal, write_signal));
        assert_eq!(reactive.get_untracked(), 1);
    }

    #[test]
    fn from_signal_t_set() {
        let (read_signal, write_signal): (ReadSignal<usize>, WriteSignal<usize>) = signal(1);
        let reactive: Reactive<usize> = Reactive::from((read_signal, write_signal));
        reactive.set(2);
        assert_eq!(reactive.get_untracked(), 2);
    }

    #[test]
    fn from_signal_t_update() {
        let (read_signal, write_signal): (ReadSignal<usize>, WriteSignal<usize>) = signal(1);
        let reactive: Reactive<usize> = Reactive::from((read_signal, write_signal));
        reactive.update(|v| *v += 1);
        assert_eq!(reactive.get_untracked(), 2);
    }

    #[test]
    fn from_signal_t_reactive() {
        let (read_signal, write_signal): (ReadSignal<usize>, WriteSignal<usize>) = signal(1);
        let reactive: Reactive<usize> = Reactive::from((read_signal, write_signal));
        write_signal.set(2);
        assert_eq!(reactive.get_untracked(), 2);
    }

    // Memo<T>
    #[test]
    fn from_memo_t_get() {
        let memo: Memo<usize> = Memo::new(move |_| 1);
        let reactive: Reactive<usize> = Reactive::from(memo);
        assert_eq!(reactive.get(), 1);
    }

    #[test]
    fn from_memo_t_get_untracked() {
        let memo: Memo<usize> = Memo::new(move |_| 1);
        let reactive: Reactive<usize> = Reactive::from(memo);
        assert_eq!(reactive.get_untracked(), 1);
    }

    #[test]
    fn from_memo_t_set() {
        let memo: Memo<usize> = Memo::new(move |_| 1);
        let reactive: Reactive<usize> = Reactive::from(memo);
        reactive.set(2);
        assert_eq!(reactive.get_untracked(), 2);
    }

    #[test]
    fn from_memo_t_update() {
        let memo: Memo<usize> = Memo::new(move |_| 1);
        let reactive: Reactive<usize> = Reactive::from(memo);
        reactive.update(|v| *v += 1);
        assert_eq!(reactive.get_untracked(), 2);
    }

    // TODO: Memo reactivity
    // #[test]
    // fn from_memo_t_reactive() {
    //     let rw_signal: RwSignal<usize> = RwSignal::new(1);
    //     let memo: Memo<usize> = Memo::new(move |_| rw_signal.get());
    //     let reactive: Reactive<usize> = Reactive::from(memo);
    //     rw_signal.update(|v| *v += 1);
    //     assert_eq!(reactive.get_untracked(), 2);
    // }

    // Two-way
    #[test]
    fn two_way_rwsignal() {
        let rw_signal: RwSignal<usize> = RwSignal::new(1);
        let reactive: Reactive<usize> = rw_signal.into();
        assert_eq!(rw_signal.get_untracked(), 1);
        assert_eq!(reactive.get_untracked(), 1);
        reactive.set(2);
        assert_eq!(reactive.get_untracked(), 2);
        assert_eq!(rw_signal.get_untracked(), 2);
    }

    #[test]
    fn two_way_signal() {
        let (read_signal, write_signal): (ReadSignal<usize>, WriteSignal<usize>) = signal(1);
        let reactive: Reactive<usize> = Reactive::from((read_signal, write_signal));
        assert_eq!(read_signal.get_untracked(), 1);
        assert_eq!(reactive.get_untracked(), 1);
        reactive.set(2);
        assert_eq!(reactive.get_untracked(), 2);
        assert_eq!(read_signal.get_untracked(), 2);
    }
}
