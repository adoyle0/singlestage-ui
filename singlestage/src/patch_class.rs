 use leptos::{
    prelude::{Get, Oco, RenderEffect, Signal, TextProp},
    tachys::{
        html::class::IntoClass,
        renderer::{
            Rndr,
            dom::{ClassList, Element},
        },
    },
};

/// A CSS class whose value lives in a **signal** that produces a `String`.
#[derive(Debug, Default, Clone)]
pub struct PatchClass(pub TextProp);

impl<S> From<S> for PatchClass
where S: Into<TextProp>
{
    fn from(value: S) -> Self {
        Self(value.into())
    }
}
impl IntoClass for PatchClass 
 {
    type AsyncOutput = Self;
    type State = RenderEffect<(Element, Oco<'static, str>)>;
    type Cloneable = Self;
    type CloneableOwned = Self;

    const MIN_LENGTH: usize = 0;

    fn html_len(&self) -> usize {
        0
    }

    fn to_html(self, class: &mut String) {
        class.push_str(&self.0.get());
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &Element) -> Self::State {
        let class_list = Rndr::class_list(el);
        let signal = self.0;
        let el = el.clone();
        RenderEffect::new(move |prev: Option<(Element, Oco<'static, str>)>| {
            let value = signal.get();
            let tokens = split_tokens(&value);

            if let Some((element, old_value)) = prev {
                let old_tokens = split_tokens(&old_value);
                diff_and_apply(&class_list, &old_tokens, &tokens);
                (element, value)
            } else {
                if !FROM_SERVER {
                    for token in &tokens {
                        Rndr::add_class(&class_list, token);
                    }
                }
                (el.clone(), value)
            }
        })
    }

    fn build(self, el: &Element) -> Self::State {
        let class_list = Rndr::class_list(el);
        let signal = self.0;
        let el = el.clone();
        RenderEffect::new(move |prev: Option<(Element, Oco<'static, str>)>| {
            let value = signal.get();
            let tokens = split_tokens(&value);

            if let Some((element, old_value)) = prev {
                let old_tokens = split_tokens(&old_value);
                diff_and_apply(&class_list, &old_tokens, &tokens);
                (element, value)
            } else {
                for token in &tokens {
                    Rndr::add_class(&class_list, token);
                }
                (el.clone(), value)
            }
        })
    }

    fn rebuild(self, state: &mut Self::State) {
        let prev = state.take_value();
        let signal = self.0;
        *state = RenderEffect::new_with_value(
            move |prev: Option<(Element, Oco<'static, str>)>| {
                let value = signal.get();
                let tokens = split_tokens(&value);

                let (element, old_value) = prev.unwrap();
                let old_tokens = split_tokens(&old_value);
                let class_list = Rndr::class_list(&element);
                diff_and_apply(&class_list, &old_tokens, &tokens);
                (element, value)
            },
            prev,
        );
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }

    fn dry_resolve(&mut self) {}

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }

    fn reset(state: &mut Self::State) {
        *state = RenderEffect::new_with_value(
            move |prev| {
                if let Some((element, value)) = prev {
                    Rndr::remove_attribute(&element, "class");
                    (element, value)
                } else {
                    unreachable!("reset called without state")
                }
            },
            state.take_value(),
        );
    }
}

#[inline]
fn diff_and_apply(class_list: &ClassList, old_tokens: &[&str], new_tokens: &[&str]) {
    for token in old_tokens.iter() {
        if !new_tokens.contains(token) {
            Rndr::remove_class(class_list, token);
        }
    }

    for token in new_tokens.iter() {
        Rndr::add_class(class_list, token);
    }
}

#[inline]
fn split_tokens(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}
