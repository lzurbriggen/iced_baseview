use baseview::Event as BaseEvent;
use iced_core::keyboard;
use iced_core::keyboard::key::Named;
use iced_core::keyboard::Location;
use iced_core::window;
use iced_core::SmolStr;
use iced_runtime::core::mouse::Button as IcedMouseButton;
use iced_runtime::core::mouse::Event as IcedMouseEvent;
use iced_runtime::core::window::Event as IcedWindowEvent;
use iced_runtime::core::Event as IcedEvent;
use iced_runtime::core::Point;
use iced_runtime::keyboard::Event as IcedKeyEvent;
use iced_runtime::keyboard::Modifiers as IcedModifiers;
use keyboard_types::Modifiers as BaseviewModifiers;

pub fn baseview_to_iced_events(
    event: BaseEvent,
    iced_events: &mut Vec<IcedEvent>,
    iced_modifiers: &mut IcedModifiers,
    ignore_non_modifier_keys: bool,
) {
    match event {
        BaseEvent::Mouse(mouse_event) => match mouse_event {
            baseview::MouseEvent::CursorMoved {
                position,
                modifiers,
            } => {
                if let Some(event) = update_modifiers(iced_modifiers, modifiers) {
                    iced_events.push(event);
                }
                iced_events.push(IcedEvent::Mouse(IcedMouseEvent::CursorMoved {
                    position: Point::new(position.x as f32, position.y as f32),
                }));
            }
            baseview::MouseEvent::ButtonPressed { button, modifiers } => {
                if let Some(event) = update_modifiers(iced_modifiers, modifiers) {
                    iced_events.push(event);
                }
                iced_events.push(IcedEvent::Mouse(IcedMouseEvent::ButtonPressed(
                    baseview_mouse_button_to_iced(button),
                )));
            }
            baseview::MouseEvent::ButtonReleased { button, modifiers } => {
                if let Some(event) = update_modifiers(iced_modifiers, modifiers) {
                    iced_events.push(event);
                }
                iced_events.push(IcedEvent::Mouse(IcedMouseEvent::ButtonReleased(
                    baseview_mouse_button_to_iced(button),
                )));
            }
            baseview::MouseEvent::WheelScrolled { delta, modifiers } => match delta {
                baseview::ScrollDelta::Lines { x, y } => {
                    if let Some(event) = update_modifiers(iced_modifiers, modifiers) {
                        iced_events.push(event);
                    }
                    iced_events.push(IcedEvent::Mouse(IcedMouseEvent::WheelScrolled {
                        delta: iced_runtime::core::mouse::ScrollDelta::Lines { x, y },
                    }));
                }
                baseview::ScrollDelta::Pixels { x, y } => {
                    if let Some(event) = update_modifiers(iced_modifiers, modifiers) {
                        iced_events.push(event);
                    }
                    iced_events.push(IcedEvent::Mouse(IcedMouseEvent::WheelScrolled {
                        delta: iced_runtime::core::mouse::ScrollDelta::Pixels { x, y },
                    }));
                }
            },
            _ => {}
        },

        BaseEvent::Keyboard(event) => {
            if let Some(event) = update_modifiers(iced_modifiers, event.modifiers) {
                iced_events.push(event);
            }

            if ignore_non_modifier_keys {
                return;
            }

            let is_down = match event.state {
                keyboard_types::KeyState::Down => true,
                keyboard_types::KeyState::Up => false,
            };

            let (key, location, text) = baseview_to_iced_keyevent(event);

            if is_down {
                iced_events.push(IcedEvent::Keyboard(keyboard::Event::KeyPressed {
                    key,
                    modifiers: *iced_modifiers,
                    location,
                    text,
                }));

                // TODO: needed?
                // if let keyboard_types::Key::Character(written) = event.key {
                //     for chr in written.chars() {
                //         iced_events.push(IcedEvent::Keyboard(IcedKeyEvent::CharacterReceived(chr)));
                //     }
                // }
            } else {
                iced_events.push(IcedEvent::Keyboard(IcedKeyEvent::KeyReleased {
                    key,
                    modifiers: *iced_modifiers,
                    location,
                }));
            }
        }

        BaseEvent::Window(window_event) => match window_event {
            baseview::WindowEvent::Resized(window_info) => {
                iced_events.push(IcedEvent::Window(
                    window::Id::MAIN,
                    IcedWindowEvent::Resized {
                        width: window_info.logical_size().width as u32,
                        height: window_info.logical_size().height as u32,
                    },
                    // TODO: this is just for the compile to work
                ));
            }
            baseview::WindowEvent::Unfocused => {
                *iced_modifiers = IcedModifiers::empty();
            }
            _ => {}
        },
    }
}

fn update_modifiers(
    iced_modifiers: &mut IcedModifiers,
    baseview_modifiers: keyboard_types::Modifiers,
) -> Option<IcedEvent> {
    let mut new = IcedModifiers::default();

    new.set(
        IcedModifiers::ALT,
        baseview_modifiers.contains(BaseviewModifiers::ALT),
    );
    new.set(
        IcedModifiers::CTRL,
        baseview_modifiers.contains(BaseviewModifiers::CONTROL),
    );
    new.set(
        IcedModifiers::SHIFT,
        baseview_modifiers.contains(BaseviewModifiers::SHIFT),
    );
    new.set(
        IcedModifiers::LOGO,
        baseview_modifiers.contains(BaseviewModifiers::META),
    );

    if *iced_modifiers != new {
        *iced_modifiers = new;

        Some(IcedEvent::Keyboard(
            iced_runtime::core::keyboard::Event::ModifiersChanged(*iced_modifiers),
        ))
    } else {
        None
    }
}

fn baseview_mouse_button_to_iced(id: baseview::MouseButton) -> IcedMouseButton {
    use baseview::MouseButton;

    match id {
        MouseButton::Left => IcedMouseButton::Left,
        MouseButton::Middle => IcedMouseButton::Middle,
        MouseButton::Right => IcedMouseButton::Right,
        MouseButton::Back => IcedMouseButton::Other(6),
        MouseButton::Forward => IcedMouseButton::Other(7),
        MouseButton::Other(other_id) => IcedMouseButton::Other(other_id as u16),
    }
}

/*
/// Converts a physical cursor position to a logical `Point`.
pub fn cursor_position(position: PhyPoint, scale_factor: f64) -> Point {
    Point::new(
        (f64::from(position.x) * scale_factor) as f32,
        (f64::from(position.y) * scale_factor) as f32,
    )
}
*/

/*
// As defined in: http://www.unicode.org/faq/private_use.html
fn is_private_use_character(c: char) -> bool {
    match c {
        '\u{E000}'..='\u{F8FF}'
        | '\u{F0000}'..='\u{FFFFD}'
        | '\u{100000}'..='\u{10FFFD}' => true,
        _ => false,
    }
}
*/

fn baseview_to_iced_key_location(location: keyboard_types::Location) -> Location {
    match location {
        keyboard_types::Location::Standard => Location::Standard,
        keyboard_types::Location::Left => Location::Left,
        keyboard_types::Location::Right => Location::Right,
        keyboard_types::Location::Numpad => Location::Numpad,
    }
}

fn baseview_to_iced_keyevent(
    event: keyboard_types::KeyboardEvent,
) -> (iced_core::keyboard::Key, Location, Option<SmolStr>) {
    use iced_core::keyboard::Key as ICode;
    use keyboard_types::Key as KCode;

    let location = baseview_to_iced_key_location(event.location);

    match event.key {
        KCode::Character(char) => (ICode::Character(char.into()), location, Some(char.into())),

        KCode::Escape => (ICode::Named(Named::Escape), location, None),

        KCode::F1 => (ICode::Named(Named::F1), location, None),
        KCode::F2 => (ICode::Named(Named::F2), location, None),
        KCode::F3 => (ICode::Named(Named::F3), location, None),
        KCode::F4 => (ICode::Named(Named::F4), location, None),
        KCode::F5 => (ICode::Named(Named::F5), location, None),
        KCode::F6 => (ICode::Named(Named::F6), location, None),
        KCode::F7 => (ICode::Named(Named::F7), location, None),
        KCode::F8 => (ICode::Named(Named::F8), location, None),
        KCode::F9 => (ICode::Named(Named::F9), location, None),
        KCode::F10 => (ICode::Named(Named::F10), location, None),
        KCode::F11 => (ICode::Named(Named::F11), location, None),
        KCode::F12 => (ICode::Named(Named::F12), location, None),

        KCode::PrintScreen => (ICode::Named(Named::PrintScreen), location, None),
        KCode::ScrollLock => (ICode::Named(Named::ScrollLock), location, None),
        KCode::Pause => (ICode::Named(Named::Pause), location, None),

        KCode::Insert => (ICode::Named(Named::Insert), location, None),
        KCode::Home => (ICode::Named(Named::Home), location, None),
        KCode::Delete => (ICode::Named(Named::Delete), location, None),
        KCode::End => (ICode::Named(Named::End), location, None),
        KCode::PageDown => (ICode::Named(Named::PageDown), location, None),
        KCode::PageUp => (ICode::Named(Named::PageUp), location, None),

        KCode::ArrowLeft => (ICode::Named(Named::ArrowLeft), location, None),
        KCode::ArrowUp => (ICode::Named(Named::ArrowUp), location, None),
        KCode::ArrowRight => (ICode::Named(Named::ArrowRight), location, None),
        KCode::ArrowDown => (ICode::Named(Named::ArrowDown), location, None),

        KCode::Backspace => (ICode::Named(Named::Backspace), location, None),
        KCode::Enter => (ICode::Named(Named::Enter), location, None),
        KCode::NumLock => (ICode::Named(Named::NumLock), location, None),
        KCode::Enter => (ICode::Named(Named::Enter), location, None),

        //KCode::AbntC1 => (ICode::AbntC1),    // TODO, loc, None)?
        //KCode::AbntC1 => (ICode::AbntC1),    // TODO, loc, None)?
        // KCode::Convert => (ICode::Convert, loc, None),
        // KCode::KanaMode => (ICode::Kana, loc, None),
        //KCode::Kanji => (ICode::Kanji),    // TODO , loc, None)?
        // KCode::NonConvert => (ICode::NoConvert, loc, None),
        // KCode::IntlYen => (ICode::Yen, loc, None),
        KCode::Alt => (ICode::Named(Named::Alt), location, None),
        KCode::Control => (ICode::Named(Named::Control), location, None),
        KCode::Shift => (ICode::Named(Named::Shift), location, None),
        KCode::Meta => (ICode::Named(Named::Meta), location, None),

        KCode::Tab => (ICode::Named(Named::Tab), location, None),
        //KCode::Underline => (ICode::Underline),    // TODO, loc, None)?
        KCode::Copy => (ICode::Named(Named::Copy), location, None),
        KCode::Paste => (ICode::Named(Named::Paste), location, None),
        KCode::Cut => (ICode::Named(Named::Cut), location, None),

        // KCode::MediaSelect => (ICode::Named(Named::Media), loc, None),
        KCode::MediaStop => (ICode::Named(Named::MediaStop), location, None),
        KCode::MediaPlayPause => (ICode::Named(Named::MediaPlayPause), location, None),
        KCode::AudioVolumeMute => (ICode::Named(Named::AudioVolumeMute), location, None),
        KCode::AudioVolumeDown => (ICode::Named(Named::AudioVolumeDown), location, None),
        KCode::AudioVolumeUp => (ICode::Named(Named::AudioVolumeUp), location, None),
        KCode::MediaTrackNext => (ICode::Named(Named::MediaTrackNext), location, None),
        KCode::MediaTrackPrevious => (ICode::Named(Named::MediaTrackPrevious), location, None),

        _ => (ICode::Unidentified, location, None),
    }
}
