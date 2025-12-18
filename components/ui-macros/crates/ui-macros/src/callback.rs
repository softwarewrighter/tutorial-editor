//! Callback builder macros for Yew applications

/// Creates a callback that sets a boolean state to true
///
/// # Example
/// ```ignore
/// let show_form = use_state(|| false);
/// let on_click = show_callback!(show_form);
/// ```
#[macro_export]
macro_rules! show_callback {
    ($state:expr) => {{
        let state = $state.clone();
        yew::Callback::from(move |_: yew::MouseEvent| state.set(true))
    }};
}

/// Creates a callback that sets a boolean state to false
///
/// # Example
/// ```ignore
/// let show_form = use_state(|| true);
/// let on_cancel = hide_callback!(show_form);
/// ```
#[macro_export]
macro_rules! hide_callback {
    ($state:expr) => {{
        let state = $state.clone();
        yew::Callback::from(move |_| state.set(false))
    }};
}

/// Creates a callback that sets a state to a specific value
///
/// # Example
/// ```ignore
/// let count = use_state(|| 0);
/// let reset = set_callback!(count, 0);
/// ```
#[macro_export]
macro_rules! set_callback {
    ($state:expr, $value:expr) => {{
        let state = $state.clone();
        yew::Callback::from(move |_| state.set($value))
    }};
}

/// Creates a callback that sets an Option state to Some(value)
///
/// # Example
/// ```ignore
/// let selected = use_state(|| None::<i64>);
/// let on_select: Callback<i64> = some_callback!(selected);
/// ```
#[macro_export]
macro_rules! some_callback {
    ($state:expr) => {{
        let state = $state.clone();
        yew::Callback::from(move |value| state.set(Some(value)))
    }};
}

/// Creates a callback that sets an Option state to None
///
/// # Example
/// ```ignore
/// let selected = use_state(|| Some(42));
/// let on_clear = none_callback!(selected);
/// ```
#[macro_export]
macro_rules! none_callback {
    ($state:expr) => {{
        let state = $state.clone();
        yew::Callback::from(move |_| state.set(None))
    }};
}

/// Creates a callback that increments a u32 state (for refresh triggers)
///
/// # Example
/// ```ignore
/// let refresh = use_state(|| 0u32);
/// let trigger_refresh = increment_callback!(refresh);
/// ```
#[macro_export]
macro_rules! increment_callback {
    ($state:expr) => {{
        let state = $state.clone();
        yew::Callback::from(move |_| state.set(*state + 1))
    }};
}
