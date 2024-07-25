use raylib::core::*;
use raylib::prelude::*;
use winit::{
    event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub(crate) enum HandledEvent {
    Keyboard(KeyboardInput),
    // Maybe used in the future
    /*MouseButton {
        button: ButtonId,
        state: ElementState,
    },
    MouseScroll(MouseScrollDelta),*/
}

fn main() {
    let width = 200;
    let height = 200;
    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("TYPESOFASTJUTSU")
        .undecorated()
        .transparent()
        .build();

    let screen_width = get_monitor_width(0);
    let screen_height = get_monitor_height(0);

    let padding = 10;
    let window_x = screen_width - width - 10;
    let window_y = screen_height * padding / 100;
    rl.set_window_position(window_x, window_y);

    let mut images = vec![];
    for index in 1..=14 {
        let mut filename = String::from("images/frame");
        let mut asserts_path = String::from("./");
        filename.push_str(index.to_string().as_str());
        filename.push_str(".png");
        asserts_path.push_str(filename.as_str());

        let texture = rl.load_texture(&thread, filename.as_str()).unwrap_or(
            rl.load_texture(&thread, asserts_path.as_str())
                .expect("Error: Fatal error loading images"),
        );
        //expect(format!("Could not load image {}", filename).as_str());
        images.push(texture);
    }

    let first = images[0].as_ref();

    let frame_width = first.width as f32;
    let frame_height = first.height as f32;
    let scale = 0.1;
    let src_rect = ffi::Rectangle {
        x: 0.0,
        y: 0.0,
        width: frame_width,
        height: frame_height,
    };
    let dest_rect = ffi::Rectangle {
        x: (width / 2) as f32,
        y: (height / 2) as f32,
        width: frame_width / 5.0,
        height: frame_height / 5.0,
    };

    // let origin = ffi::Vector2 {x: (frame_width * scale) as f32,  y: (frame_height * scale) as f32};
    let origin = ffi::Vector2 {
        x: (frame_width * scale),
        y: (frame_height * scale),
    };

    rl.set_target_fps(60);
    let mut index = 0;
    let mut display = String::from("[");
    let mut text_x = width / 2;

    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).expect("Could not create x11 window");
    window.set_visible(false);

    let mut ctrl_down = false;

    // let (tx, _) = channel::unbounded();
    while !rl.window_should_close() {
        let mut message = String::from("");
        event_loop.run(move |event, _, control_flow| {
            // *control_flow = ControlFlow::Wait;
            // let tx = tx.clone();
            match event {
                Event::DeviceEvent {
                    event: DeviceEvent::Key(key),
                    ..
                } => {
                    let current_key = key;
                    if current_key.state == ElementState::Pressed {
                        return;
                    }

                    let code = current_key.virtual_keycode.unwrap_or(VirtualKeyCode::Space);
                    // task::spawn(async move { tx.send(HandledEvent::Keyboard(key)).await });
                    match code {
                        VirtualKeyCode::A => {
                            index = 13;
                            message.push('a');
                        }
                        VirtualKeyCode::B => {
                            index = 1;
                            message.push('b');
                        }
                        VirtualKeyCode::C => {
                            index = 2;
                            message.push('c');
                        }
                        VirtualKeyCode::D => {
                            index = 3;
                            message.push('d');
                        }
                        VirtualKeyCode::E => {
                            index = 4;
                            message.push('e');
                        }
                        VirtualKeyCode::F => {
                            index = 5;
                            message.push('f');
                        }
                        VirtualKeyCode::G => {
                            index = 6;
                            message.push('g');
                        }
                        VirtualKeyCode::H => {
                            index = 7;
                            message.push('h');
                        }
                        VirtualKeyCode::I => {
                            index = 9;
                            message.push('i');
                        }
                        VirtualKeyCode::J => {
                            index = 8;
                            message.push('j');
                        }
                        VirtualKeyCode::K => {
                            index = 9;
                            message.push('k');
                        }
                        VirtualKeyCode::L => {
                            index = 10;
                            message.push('l');
                        }
                        VirtualKeyCode::M => {
                            index = 11;
                            message.push('m');
                        }
                        VirtualKeyCode::N => {
                            index = 12;
                            message.push('n');
                        }
                        VirtualKeyCode::O => {
                            index = 13;
                            message.push('o');
                        }
                        VirtualKeyCode::P => {
                            index = 1;
                            message.push('p');
                        }
                        VirtualKeyCode::Q => {
                            index = 2;
                            message.push('q');
                        }
                        VirtualKeyCode::R => {
                            index = 3;
                            message.push('r');
                        }
                        VirtualKeyCode::S => {
                            index = 4;
                            message.push('s');
                        }
                        VirtualKeyCode::T => {
                            index = 5;
                            message.push('t');
                        }
                        VirtualKeyCode::U => {
                            index = 6;
                            message.push('u');
                        }
                        VirtualKeyCode::V => {
                            index = 7;
                            message.push('v');
                        }
                        VirtualKeyCode::W => {
                            index = 8;
                            message.push('w');
                        }
                        VirtualKeyCode::X => {
                            index = 9;
                            message.push('x');
                        }
                        VirtualKeyCode::Y => {
                            index = 10;
                            message.push('y');
                        }
                        VirtualKeyCode::Z => {
                            index = 11;
                            message.push('z');
                        }
                        VirtualKeyCode::Return => {
                            index = 12;
                            message.push_str("[e]");
                        }
                        VirtualKeyCode::Space => {
                            index = 13;
                            message.push_str("[_]");
                        }
                        VirtualKeyCode::Escape => {
                            index = 0;
                            message.push_str("<esc>");
                        }
                        VirtualKeyCode::Left => {
                            index = 1;
                            message.push_str("<left>");
                        }
                        VirtualKeyCode::Right => {
                            index = 2;
                            message.push_str("<right>");
                        }
                        VirtualKeyCode::Up => {
                            index = 3;
                            message.push_str("<Up>");
                        }
                        VirtualKeyCode::Down => {
                            index = 4;
                            message.push_str("<down>");
                        }
                        VirtualKeyCode::Semicolon => {
                            index = 5;
                            message.push(';');
                        }
                        VirtualKeyCode::Period => {
                            index = 6;
                            message.push('.');
                        }
                        VirtualKeyCode::Apostrophe => {
                            index = 7;
                            message.push('\'');
                        }
                        VirtualKeyCode::Tab => {
                            index = 8;
                            message.push_str("[TAB]");
                        }
                        VirtualKeyCode::LBracket => {
                            index = 9;
                            message.push('[');
                        }
                        VirtualKeyCode::RBracket => {
                            index = 10;
                            message.push(']');
                        }
                        VirtualKeyCode::Back => {
                            index = 11;
                            message.push_str("[BACK]");
                        }
                        VirtualKeyCode::Key1 => {
                            index = 12;
                            message.push('1');
                        }
                        VirtualKeyCode::Key2 => {
                            index = 13;
                            message.push('2');
                        }
                        VirtualKeyCode::Key3 => {
                            index = 0;
                            message.push('3');
                        }
                        VirtualKeyCode::Key4 => {
                            index = 1;
                            message.push('4');
                        }
                        VirtualKeyCode::Key5 => {
                            index = 2;
                            message.push('5');
                        }
                        VirtualKeyCode::Key6 => {
                            index = 3;
                            message.push('6');
                        }
                        VirtualKeyCode::Key7 => {
                            index = 4;
                            message.push('7');
                        }
                        VirtualKeyCode::Key8 => {
                            index = 5;
                            message.push('8');
                        }
                        VirtualKeyCode::Key9 => {
                            index = 6;
                            message.push('9');
                        }
                        VirtualKeyCode::Key0 => {
                            index = 7;
                            message.push('0');
                        }
                        VirtualKeyCode::Comma => {
                            index = 8;
                            message.push(',');
                        }
                        VirtualKeyCode::LControl | VirtualKeyCode::RControl => {
                            index = 9;
                            message.push_str("[CTRL]");

                            ctrl_down = !ctrl_down;

                            println!("CTRL key_down? {}", ctrl_down);
                        }
                        VirtualKeyCode::Delete => {
                            index = 10;
                            message.push_str("[DEL]");
                            if ctrl_down {
                                *control_flow = ControlFlow::Exit;
                                std::process::exit(1);
                            }
                        }
                        _ => {
                            println!("unhandled key: {:?}", code);
                        }
                    }
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                    println!("Exit pressed");
                }
                _ => {
                    message = String::from("");
                }
            }

            /*if index > 0 {
                index = index - 1
            } */

            if text_x < 0 {
                text_x = width / 2;
                display = "".to_string();
            }

            if !message.is_empty() {
                display.push_str(message.as_str());
                text_x -= 5;
            }
            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::BLACK);
            d.draw_texture_pro(
                &images[index],
                src_rect,
                dest_rect,
                origin,
                0.0,
                Color::WHITE,
            );
            d.draw_text(display.as_str(), text_x, height - 20, 16, Color::WHITE);
        });
    }
}
