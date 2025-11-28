use leptos::prelude::*;
use std::fmt::Debug;
use reactive_stores::{ArcField, Field, StoreField, Subfield};

/// A reactive binding wrapper that can take any value and upgrade it to a RwSignal.
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
#[derive(Clone, Debug)]
pub struct Reactive<T, U = RwSignal<T>>(U)
where
    U: IntoReactive<Value = T>;

impl<T, U: IntoReactive<Value = T> + Copy> Reactive<T, U> {
    #[inline]
    pub fn get(&self) -> T {
        self.0.into_reactive().0.get()
    }

    #[inline]
    pub fn get_untracked(&self) -> T {
        self.0.into_reactive().0.get_untracked()
    }

    #[inline]
    pub fn set(&self, value: T) {
        self.0.into_reactive().1.set(value);
    }

    #[inline]
    pub fn with<K>(&self, fun: impl FnOnce(&T) -> K) -> K {
        self.0.with(fun)
    }

    #[inline]
    pub fn update(&self, fun: impl FnOnce(&mut T)) {
        self.0.update(fun);
    }
}

impl<T: Clone, U: Copy + IntoReactive<Value = T>> Copy for Reactive<T, U> {}

impl<T: Default + Send + Clone + Sync + 'static> Default for Reactive<T> {
    fn default() -> Self {
        Self(RwSignal::<T>::new(Default::default()))
    }
}

impl<T: Default + Send + Clone + Sync + 'static> Reactive<T> {
    pub fn new(value: T) -> Self {
        Self(RwSignal::<T>::new(value))
    }
}

impl From<&str> for Reactive<String> {
    fn from(value: &str) -> Self {
        Reactive(RwSignal::new(value.to_string()))
    }
}

impl<T> From<T> for Reactive<T>
where
    T: Send + Sync + Clone + 'static,
{
    fn from(value: T) -> Self {
        Reactive(RwSignal::new(value))
    }
}

impl<U, T> From<U> for Reactive<T, U>
where
    U: IntoReactive<Value = T>,
{
    fn from(value: U) -> Self {
        Reactive(value)
    }
}

pub trait IntoReactive {
    type Value;
    type Get: Get<Value = Self::Value> + GetUntracked<Value = Self::Value>;
    type Set: Set<Value = Self::Value>;

    fn into_reactive(self) -> (Self::Get, Self::Set);
    fn with<U>(&self, fun: impl FnOnce(&Self::Value) -> U) -> U;
    fn update(&self, fun: impl FnOnce(&mut Self::Value));
}

impl<T> IntoReactive for RwSignal<T>
where
    T: Send + Sync + 'static + Clone,
    ReadSignal<T>: GetUntracked<Value = T>,
    Self: DefinedAt,
{
    type Value = T;
    type Get = ReadSignal<T>;
    type Set = WriteSignal<T>;

    fn into_reactive(self) -> (ReadSignal<T>, WriteSignal<T>) {
        self.split()
    }

    fn with<U>(&self, fun: impl FnOnce(&Self::Value) -> U) -> U {
        With::with(self, fun)
    }

    fn update(&self, fun: impl FnOnce(&mut Self::Value)) {
        Update::update(self, fun);
    }
}

impl<T, Inner, Prev> IntoReactive for Subfield<Inner, Prev, T>
where
    T: Send + Sync + 'static + Clone,
    Inner: StoreField<Value = Prev> + Send + Sync + Clone + 'static,
    Prev: 'static,
    Subfield<Inner, Prev, T>: Track + IsDisposed,
    ReadSignal<T>: GetUntracked<Value = T>,
    Self: DefinedAt,
{
    type Value = T;
    type Get = Subfield<Inner, Prev, T>;
    type Set = Subfield<Inner, Prev, T>;

    fn into_reactive(self) -> (Subfield<Inner, Prev, T>, Subfield<Inner, Prev, T>) {
        (self.clone(), self)
    }

    fn with<U>(&self, fun: impl FnOnce(&Self::Value) -> U) -> U {
        With::with(self, fun)
    }

    fn update(&self, fun: impl FnOnce(&mut Self::Value)) {
        Update::update(self, fun);
    }
}
