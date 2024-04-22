pub mod app_event;
pub mod key_event;
pub mod mouse_event;

enum EvenType {
    Nil,
    WinClose,
    WinResize,
    WinFocus,
    WinLostFocus,
    WinMoved,
    AppTick,
    AppUpdate,
    AppRender,
    KeyPressed,
    KeyReleased,
    MouseButtonPressed,
    MouseButtonReleased,
    MouseMoved,
    MouseScrolled,
}

enum EventCategory {
    Nil,
    App,
    Input,
    Keyboard,
    Mouse,
    MouseButton,
}

trait Event {

}

trait AppEvent {

}

trait InputEvent {

}

trait MouseEvent {

}

trait MouseButtonEvent {

}