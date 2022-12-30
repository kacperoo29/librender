// TODO: Replace unwraps
extern crate glfw;

use std::{
    collections::HashMap,
    ptr,
    sync::{mpsc::Receiver, Mutex},
};

use glfw::{Context, Glfw, WindowEvent};

use ::once_cell::sync::Lazy;

use crate::{
    event::{Event, Keycode, MouseButton},
    render::window::Window,
};

static EVENT_MAP: Lazy<Mutex<HashMap<usize, Vec<Event>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub struct GlfwWindow {
    window: Option<glfw::Window>,
    glfw: Glfw,
    event_reciever: Receiver<(f64, WindowEvent)>,
}

impl GlfwWindow {
    pub fn new() -> Box<dyn Window> {
        let glfw = glfw::init(Some(glfw::Callback {
            f: GlfwWindow::error_callback,
            data: String::new(),
        }))
        .expect("Failed to init GLFW");

        let (mut window, events) = glfw
            .create_window(800, 600, "title", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_all_polling(true);
        

        unsafe {
            glfw::ffi::glfwSetCursorPosCallback(
                window.window_ptr(),
                Some(GlfwWindow::cursor_pos_callback),
            );
        }

        EVENT_MAP
            .lock()
            .unwrap()
            .insert(window.window_ptr() as usize, Vec::new());

        return Box::new(GlfwWindow {
            window: Some(window),
            event_reciever: events,
            glfw: glfw,
        });
    }

    fn error_callback(_error: glfw::Error, _str: String, _user_data: &String) {}

    fn translate_keycode(keycode: glfw::Key) -> Option<Keycode> {
        match keycode {
            glfw::Key::W => Some(Keycode::W),
            glfw::Key::A => Some(Keycode::A),
            glfw::Key::S => Some(Keycode::S),
            glfw::Key::D => Some(Keycode::D),
            glfw::Key::Space => Some(Keycode::Space),
            glfw::Key::LeftShift => Some(Keycode::LeftShift),
            glfw::Key::Escape => Some(Keycode::Escape),
            _ => None,
        }
    }

    fn translate_keycode_mouse(button: glfw::MouseButton) -> Option<MouseButton> {
        match button {
            glfw::MouseButton::Button1 => Some(MouseButton::Left),
            glfw::MouseButton::Button2 => Some(MouseButton::Right),
            glfw::MouseButton::Button3 => Some(MouseButton::Middle),
            _ => None,
        }
    }

    extern "C" fn cursor_pos_callback(window: *mut glfw::ffi::GLFWwindow, x: f64, y: f64) {
        let winaddr = window as usize;
        EVENT_MAP
            .lock()
            .unwrap()
            .get_mut(&winaddr)
            .unwrap()
            .push(Event::MouseMoved {
                x: x as f32,
                y: y as f32,
            });
    }
}

impl Drop for GlfwWindow {
    fn drop(&mut self) {
        match &self.window {
            Some(window) => {
                EVENT_MAP
                    .lock()
                    .unwrap()
                    .remove(&(window.window_ptr() as usize));
            }
            None => {}
        }
    }
}

impl Window for GlfwWindow {
    fn get_events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        self.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.event_reciever) {
            match event {
                WindowEvent::Key(keycode, _, glfw::Action::Press, _) => {
                    events.push(Event::KeyPressed {
                        keycode: GlfwWindow::translate_keycode(keycode),
                    })
                }
                WindowEvent::Key(keycode, _, glfw::Action::Release, _) => {
                    events.push(Event::KeyReleased {
                        keycode: GlfwWindow::translate_keycode(keycode),
                    })
                }
                WindowEvent::MouseButton(button, glfw::Action::Press, _) => {
                    events.push(Event::MousePressed {
                        button: GlfwWindow::translate_keycode_mouse(button),
                    })
                }
                WindowEvent::MouseButton(button, glfw::Action::Release, _) => {
                    events.push(Event::MouseReleased {
                        button: GlfwWindow::translate_keycode_mouse(button),
                    })
                }
                WindowEvent::Close => events.push(Event::Quit),
                _ => events.push(Event::None),
            }
        }

        // TODO: Disgusting
        let winaddr = match &self.window {
            Some(window) => window.window_ptr() as usize,
            None => 0,
        };
        let mut map = EVENT_MAP.lock().unwrap();
        let additional_events = map.get_mut(&winaddr).unwrap();
        events.append(additional_events);

        return events;
    }

    fn should_close(&self) -> bool {
        match &self.window {
            Some(window) => window.should_close(),
            None => true,
        }
    }

    fn close(&mut self) {
        match self.window.take() {
            Some(window) => {
                window.close();
            }
            None => {}
        }
    }

    fn update(&mut self) {
        match self.window.as_mut() {
            Some(window) => {
                window.swap_buffers();
            }
            None => {}
        }
    }

    fn get_proc_addr(&mut self, procname: &str) -> *const std::ffi::c_void {
        match self.window.as_mut() {
            Some(window) => window.get_proc_address(procname),
            None => ptr::null(),
        }
    }

    fn set_title(&mut self, title: &str) {
        match self.window.as_mut() {
            Some(window) => window.set_title(title),
            None => {}
        }
    }

    fn toggle_mouse_grab(&mut self) {
        match self.window.as_mut() {
            Some(window) => {
                let mode = window.get_cursor_mode();
                if mode == glfw::CursorMode::Disabled {
                    window.set_cursor_mode(glfw::CursorMode::Normal);
                } else {
                    window.set_cursor_mode(glfw::CursorMode::Disabled);
                }
            }
            None => {}
        }
    }

    fn get_size(&self) -> (f32, f32) {
        match self.window.as_ref() {
            Some(window) => {
                let (width, height) = window.get_size();
                return (width as f32, height as f32);
            }
            None => (0.0, 0.0),
        }
    }
}
