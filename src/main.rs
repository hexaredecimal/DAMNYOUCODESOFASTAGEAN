use async_std::{
    channel::{self},
    task,
};
use raylib::prelude::*;
use winit::{
    event::{
        ButtonId, DeviceEvent, ElementState, Event, KeyboardInput, MouseScrollDelta,
        VirtualKeyCode, WindowEvent,
    },
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
        .title("DAMNYCODESOFASTAGEAN")
        .build();

    let mut images = vec![];
    for index in 1..=14 {
        let mut filename = String::from("images/frame");
        let mut asserts_path = String::from("/usr/local/");
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
        x: (frame_width * scale) as f32,
        y: (frame_height * scale) as f32,
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
                    let current_key = key.clone();
                    if current_key.state == ElementState::Pressed {
                        return ();
                    }

                    let code = current_key.virtual_keycode.unwrap_or(VirtualKeyCode::Space);
                    // task::spawn(async move { tx.send(HandledEvent::Keyboard(key)).await });
                    match code {
                        VirtualKeyCode::A => {
                            index = 13;
                            message.push_str("a");
                        }
                        VirtualKeyCode::B => {
                            index = 1;
                            message.push_str("b");
                        }
                        VirtualKeyCode::C => {
                            index = 2;
                            message.push_str("c");
                        }
                        VirtualKeyCode::D => {
                            index = 3;
                            message.push_str("d");
                        }
                        VirtualKeyCode::E => {
                            index = 4;
                            message.push_str("e");
                        }
                        VirtualKeyCode::F => {
                            index = 5;
                            message.push_str("f");
                        }
                        VirtualKeyCode::G => {
                            index = 6;
                            message.push_str("g");
                        }
                        VirtualKeyCode::H => {
                            index = 7;
                            message.push_str("h");
                        }
                        VirtualKeyCode::I => {
                            index = 9;
                            message.push_str("i");
                        }
                        VirtualKeyCode::J => {
                            index = 8;
                            message.push_str("j");
                        }
                        VirtualKeyCode::K => {
                            index = 9;
                            message.push_str("k");
                        }
                        VirtualKeyCode::L => {
                            index = 10;
                            message.push_str("l");
                        }
                        VirtualKeyCode::M => {
                            index = 11;
                            message.push_str("m");
                        }
                        VirtualKeyCode::N => {
                            index = 12;
                            message.push_str("n");
                        }
                        VirtualKeyCode::O => {
                            index = 13;
                            message.push_str("o");
                        }
                        VirtualKeyCode::P => {
                            index = 1;
                            message.push_str("p");
                        }
                        VirtualKeyCode::Q => {
                            index = 2;
                            message.push_str("q");
                        }
                        VirtualKeyCode::R => {
                            index = 3;
                            message.push_str("r");
                        }
                        VirtualKeyCode::S => {
                            index = 4;
                            message.push_str("s");
                        }
                        VirtualKeyCode::T => {
                            index = 5;
                            message.push_str("t");
                        }
                        VirtualKeyCode::U => {
                            index = 6;
                            message.push_str("u");
                        }
                        VirtualKeyCode::V => {
                            index = 7;
                            message.push_str("v");
                        }
                        VirtualKeyCode::W => {
                            index = 8;
                            message.push_str("w");
                        }
                        VirtualKeyCode::X => {
                            index = 9;
                            message.push_str("x");
                        }
                        VirtualKeyCode::Y => {
                            index = 10;
                            message.push_str("y");
                        }
                        VirtualKeyCode::Z => {
                            index = 11;
                            message.push_str("z");
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
                            message.push_str(";");
                        }
                        VirtualKeyCode::Period => {
                            index = 6;
                            message.push_str(".");
                        }
                        VirtualKeyCode::Apostrophe => {
                            index = 7;
                            message.push_str("'");
                        }
                        VirtualKeyCode::Tab => {
                            index = 8;
                            message.push_str("[TAB]");
                        }
                        VirtualKeyCode::LBracket => {
                            index = 9;
                            message.push_str("[");
                        }
                        VirtualKeyCode::RBracket => {
                            index = 10;
                            message.push_str("]");
                        }
                        VirtualKeyCode::Back => {
                            index = 11;
                            message.push_str("[BACK]");
                        }
                        VirtualKeyCode::Key1 => {
                            index = 12;
                            message.push_str("1");
                        }
                        VirtualKeyCode::Key2 => {
                            index = 13;
                            message.push_str("2");
                        }
                        VirtualKeyCode::Key3 => {
                            index = 0;
                            message.push_str("3");
                        }
                        VirtualKeyCode::Key4 => {
                            index = 1;
                            message.push_str("4");
                        }
                        VirtualKeyCode::Key5 => {
                            index = 2;
                            message.push_str("5");
                        }
                        VirtualKeyCode::Key6 => {
                            index = 3;
                            message.push_str("6");
                        }
                        VirtualKeyCode::Key7 => {
                            index = 4;
                            message.push_str("7");
                        }
                        VirtualKeyCode::Key8 => {
                            index = 5;
                            message.push_str("8");
                        }
                        VirtualKeyCode::Key9 => {
                            index = 6;
                            message.push_str("9");
                        }
                        VirtualKeyCode::Key0 => {
                            index = 7;
                            message.push_str("0");
                        }
                        VirtualKeyCode::Comma => {
                            index = 8;
                            message.push_str(",");
                        }
                        VirtualKeyCode::LControl | VirtualKeyCode::RControl => {
                            index = 9;
                            message.push_str("[CTRL]"); 

                            if ctrl_down {
                                ctrl_down = false;
                            } else {
                                ctrl_down = true; 
                            }

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
