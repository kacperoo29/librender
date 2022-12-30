#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keycode {
    Unknown = 0,
    Space = 32,
    W = 87,
    A = 65,
    S = 83,
    D = 68,
    Escape = 256,
    PageUp = 266,
    PageDown = 267,
    End = 269,
    LeftShift = 340,
}

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left = 0,
    Right = 1,
    Middle = 2,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Event {
    Quit,
    KeyPressed { keycode: Option<Keycode> },
    KeyReleased { keycode: Option<Keycode> },
    MouseMoved { x: f32, y: f32 },
    MousePressed { button: Option<MouseButton> },
    MouseReleased { button: Option<MouseButton> },
    None,
}
