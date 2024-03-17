//! A windowing shell for Iced, on top of [`baseview`].
//!
//! Largely stolen from (MIT licensed) [`iced_winit`].
//!
//! [`baseview`]: https://github.com/RustAudio/baseview
//! [`iced_winit`]: https://github.com/iced-rs/iced/tree/master/winit
#![deny(
    // missing_debug_implementations,
    // missing_docs,
    unused_results,
    clippy::extra_unused_lifetimes,
    clippy::from_over_into,
    clippy::needless_borrow,
    clippy::new_without_default,
    clippy::useless_conversion,
)]
#![forbid(rust_2018_idioms)]
#![allow(clippy::inherent_to_string, clippy::type_complexity)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
use application::DefaultStyle;
use iced_core::Element;
pub use iced_futures;
// pub use iced::
// TODO: pub use iced_graphics as graphics;
// TODO: pub use iced_runtime as runtime;
// TODO: pub use iced_runtime::core;
// TODO: pub use iced_runtime::futures;
// TODO: pub use iced_style as style;
// TODO: pub use iced_widget as widget;
pub use iced_core as core;
pub use iced_runtime as runtime;
pub use iced_style as style;
pub use iced_widget as widget;

mod application;
pub mod clipboard;
pub mod conversion;
pub mod settings;
pub mod window;

#[cfg(feature = "system")]
pub mod system;

mod error;
mod position;
mod proxy;

#[cfg(feature = "trace")]
pub use application::Profiler;
pub use clipboard::Clipboard;
pub use error::Error;
use iced_futures::Executor;
use iced_futures::Subscription;
use iced_runtime::Command;
pub use position::Position;
pub use proxy::Proxy;
pub use settings::Settings;

// TODO: pub use iced_graphics::Viewport;
// TODO: pub use iced::
// TODO: use iced::::style::application::StyleSheet;

pub mod baseview {
    pub use baseview::{Size, WindowOpenOptions, WindowScalePolicy};
}

use iced_style::application::StyleSheet;

// TODO: use iced_widget::renderer;
use window::WindowSubs;

pub type Renderer = iced_renderer::Renderer;

pub mod executor {
    //! Choose your preferred executor to power your application.
    // TODO: pub use iced_runtime::futures::Executor;

    /// A default cross-platform executor.
    ///
    /// - On native platforms, it will use:
    ///   - `iced_futures::backend::native::tokio` when the `tokio` feature is enabled.
    ///   - `iced_futures::backend::native::async-std` when the `async-std` feature is
    ///     enabled.
    ///   - `iced_futures::backend::native::smol` when the `smol` feature is enabled.
    ///   - `iced_futures::backend::native::thread_pool` otherwise.
    ///
    /// - On Wasm, it will use `iced_futures::backend::wasm::wasm_bindgen`.
    pub type Default = iced_runtime::futures::backend::null::Executor;
}

pub trait Application: Sized + std::marker::Send {
    /// The [`Executor`] that will run commands and subscriptions.
    ///
    /// The [default executor] can be a good starting point!
    ///
    /// [`Executor`]: Self::Executor
    /// [default executor]: crate::executor::Default
    type Executor: Executor;

    /// The type of __messages__ your [`Application`] will produce.
    type Message: std::fmt::Debug + Send;

    /// The theme of your [`Application`].
    type Theme: Default + iced_style::application::StyleSheet;

    /// The data needed to initialize your [`Application`].
    type Flags: std::marker::Send;

    /// Initializes the [`Application`] with the flags provided to
    /// [`run`] as part of the [`Settings`].
    ///
    /// Here is where you should return the initial state of your app.
    ///
    /// Additionally, you can return a [`Command`] if you need to perform some
    /// async action in the background on startup. This is useful if you want to
    /// load state from a file, perform an initial HTTP request, etc.
    ///
    /// [`run`]: Self::run
    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>);

    /// Returns the current title of the [`Application`].
    ///
    /// This title can be dynamic! The runtime will automatically update the
    /// title of your application when necessary.
    fn title(&self) -> String;

    // FIXME: window_queue?
    /// Handles a __message__ and updates the state of the [`Application`].
    ///
    /// This is where you define your __update logic__. All the __messages__,
    /// produced by either user interactions or commands, will be handled by
    /// this method.
    ///
    /// Any [`Command`] returned will be executed immediately in the background.
    fn update(&mut self, message: Self::Message) -> Command<Self::Message>;

    /// Returns the widgets to display in the [`Application`].
    ///
    /// These widgets can produce __messages__ based on user interaction.
    fn view(&self) -> Element<'_, Self::Message, Self::Theme, crate::Renderer>;

    /// Returns the current [`Theme`] of the [`Application`].
    ///
    /// [`Theme`]: Self::Theme
    fn theme(&self) -> Self::Theme {
        Self::Theme::default()
    }

    /// Returns the current `Style` of the [`Theme`].
    ///
    /// [`Theme`]: Self::Theme
    fn style(&self) -> <Self::Theme as iced_style::application::StyleSheet>::Style {
        <Self::Theme as iced_style::application::StyleSheet>::Style::default()
    }

    /// Returns the event [`Subscription`] for the current state of the
    /// application.
    ///
    /// A [`Subscription`] will be kept alive as long as you keep returning it,
    /// and the __messages__ produced will be handled by
    /// [`update`](#tymethod.update).
    ///
    /// By default, this method returns an empty [`Subscription`].
    fn subscription(
        &self,
        _window_subs: &mut WindowSubs<Self::Message>,
    ) -> Subscription<Self::Message> {
        Subscription::none()
    }
    /// Returns the [`WindowScalePolicy`] that the [`Application`] should use.
    ///
    /// By default, it returns `WindowScalePolicy::SystemScaleFactor`.
    ///
    /// [`WindowScalePolicy`]: ../settings/enum.WindowScalePolicy.html
    /// [`Application`]: trait.Application.html
    fn scale_policy(&self) -> baseview::WindowScalePolicy {
        baseview::WindowScalePolicy::SystemScaleFactor
    }

    fn renderer_settings() -> iced_renderer::Settings {
        Default::default()
    }
}

struct Instance<A>(A) where
A: Application,
A::Theme: DefaultStyle;


impl<A> iced_runtime::Program for Instance<A>
where
    A: Application,
    A::Theme: DefaultStyle,
{
    type Message = A::Message;
    type Theme = A::Theme;
    type Renderer = crate::Renderer;

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.0.update(message)
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Self::Renderer> {
        self.0.view()
    }
}

impl<A> crate::application::Application for Instance<A>
where
    A: Application,
    A::Theme: DefaultStyle,
{
    type Flags = A::Flags;

    fn new(flags: Self::Flags) -> (Self, Command<A::Message>) {
        let (app, command) = A::new(flags);

        (Instance(app), command)
    }

    fn title(&self) -> String {
        self.0.title()
    }

    fn theme(&self) -> A::Theme {
        self.0.theme()
    }

    fn style(&self) -> <A::Theme as StyleSheet>::Style {
        self.0.style()
    }

    fn subscription(
        &self,
        window_subs: &mut WindowSubs<A::Message>,
    ) -> iced_futures::Subscription<Self::Message> {
        self.0.subscription(window_subs)
    }

    fn scale_policy(&self) -> baseview::WindowScalePolicy {
        self.0.scale_policy()
    }

    fn renderer_settings() -> iced_renderer::Settings {
        A::renderer_settings()
    }
}

/// Runs the [`Application`] in a child window.
pub fn open_parented<A, P>(
    parent: &P,
    settings: Settings<A::Flags>,
) -> window::WindowHandle<A::Message>
where
    A: Application + 'static,
    A::Theme: DefaultStyle,
    P: raw_window_handle::HasWindowHandle,
{
    window::IcedWindow::<Instance<A>>::open_parented::<A::Executor, iced_renderer::Compositor, P>(
        parent, settings,
    )
}

/// Runs the [`Application`]. Open a new window that blocks the current thread until the window is destroyed.
///
/// * `settings` - The settings of the window.
pub fn open_blocking<A>(settings: Settings<A::Flags>)
where
    A: Application + 'static,
    A::Theme: DefaultStyle
{
    window::IcedWindow::<Instance<A>>::open_blocking::<A::Executor, iced_renderer::Compositor>(
        settings,
    );
}
