use miniquad::*;

struct Stage {
    pen_info: String,
    pen_down: bool,
    stroke_count: usize,
    last_pressure: f32,
    last_position: (f32, f32),
}

impl Stage {
    fn new() -> Stage {
        Stage {
            pen_info: "No pen input detected yet".to_string(),
            pen_down: false,
            stroke_count: 0,
            last_pressure: 0.0,
            last_position: (0.0, 0.0),
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        // Simple colored background based on pen state
        let bg_color = if self.pen_down {
            (0.2, 0.8, 0.2, 1.0) // Green when pen is down
        } else {
            (0.1, 0.1, 0.2, 1.0) // Dark blue otherwise
        };

        // clear_background(bg_color.0, bg_color.1, bg_color.2, bg_color.3);

        // In a real app, you'd draw the pen_info text on screen
        // For now we'll just print important events to console
    }

    fn pen_input_event(&mut self, phase: PenPhase, tool_type: PenToolType, pen_data: PenInput) {
        self.last_pressure = pen_data.pressure;
        self.last_position = (pen_data.x, pen_data.y);

        match phase {
            PenPhase::Proximity => {
                self.pen_info = format!(
                    "Pen {:?} in proximity at ({:.1}, {:.1}) - distance: {:.3}",
                    tool_type, pen_data.x, pen_data.y, pen_data.distance
                );
                println!("🖊️  {}", self.pen_info);
            }

            PenPhase::Down => {
                self.pen_down = true;
                self.stroke_count += 1;
                self.pen_info = format!(
                    "Pen {:?} DOWN - Stroke #{} - Pressure: {:.3} - Tilt: ({:.1}°, {:.1}°) - Rotation: {:.1}°",
                    tool_type, self.stroke_count, pen_data.pressure, pen_data.tilt_x, pen_data.tilt_y, pen_data.rotation
                );
                println!("✏️  {}", self.pen_info);
            }

            PenPhase::Move => {
                self.pen_info = format!(
                    "Pen {:?} move - Pos: ({:.1}, {:.1}) - Pressure: {:.3} - Tilt: ({:.1}°, {:.1}°)",
                    tool_type, pen_data.x, pen_data.y, pen_data.pressure, pen_data.tilt_x, pen_data.tilt_y
                );

                // Only print move events occasionally to avoid spam
                if (pen_data.x as i32) % 10 == 0 || (pen_data.y as i32) % 10 == 0 {
                    println!("📍 {}", self.pen_info);
                }
            }

            PenPhase::Up => {
                self.pen_down = false;
                self.pen_info = format!(
                    "Pen {:?} UP - Final position: ({:.1}, {:.1})",
                    tool_type, pen_data.x, pen_data.y
                );
                println!("⬆️  {}", self.pen_info);
            }

            PenPhase::Leave => {
                self.pen_down = false;
                self.pen_info = format!("Pen {:?} left proximity", tool_type);
                println!("👋 {}", self.pen_info);
            }
        }
    }

    fn mouse_button_down_event(&mut self, button: MouseButton, x: f32, y: f32) {
        println!(
            "🖱️  Mouse {:?} down at ({:.1}, {:.1}) [Fallback input]",
            button, x, y
        );
        if button == MouseButton::Left {
            self.pen_down = true;
            self.stroke_count += 1;
        }
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, x: f32, y: f32) {
        println!(
            "🖱️  Mouse {:?} up at ({:.1}, {:.1}) [Fallback input]",
            button, x, y
        );
        if button == MouseButton::Left {
            self.pen_down = false;
        }
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.last_position = (x, y);
        // Don't spam mouse motion events
    }

    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, repeat: bool) {
        if repeat {
            return;
        }

        match keycode {
            KeyCode::Space => {
                self.stroke_count = 0;
                println!("🔄 Reset stroke counter");
            }
            KeyCode::I => {
                println!("ℹ️  Current state:");
                println!("   Pen down: {}", self.pen_down);
                println!("   Stroke count: {}", self.stroke_count);
                println!("   Last pressure: {:.3}", self.last_pressure);
                println!(
                    "   Last position: ({:.1}, {:.1})",
                    self.last_position.0, self.last_position.1
                );
                println!("   Info: {}", self.pen_info);
            }
            KeyCode::Escape => {
                println!("👋 Exiting pen test");
                std::process::exit(0);
            }
            _ => {}
        }
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        println!("📏 Window resized to {}x{}", width, height);
    }
}

fn main() {
    let mut conf = conf::Conf::default();
    conf.window_title = "Miniquad Pen Input Test".to_owned();
    conf.window_width = 800;
    conf.window_height = 600;
    // let mut plat = conf::Platform::default();
    // conf.platform.framebuffer_alpha = false;
    // conf.sample_count = 1;
    // plat.linux_backend = conf::LinuxBackend::WaylandWithX11Fallback;
    // // plat.linux_x11_gl = conf::LinuxX11Gl::default();
    // // plat.linux_x11_gl = conf::LinuxX11Gl::EGLOnly;
    // plat.wayland_decorations = conf::WaylandDecorations::default();
    // conf.platform = plat;
    // conf.

    println!("🖊️  Miniquad Pen Input Test");
    println!("=====================================");
    println!("This example tests pen/stylus input support in Miniquad.");
    println!("");
    println!("Instructions:");
    println!("• Use a pen/stylus to interact with the window");
    println!("• Watch the console for detailed pen input events");
    println!("• Background turns green when pen touches surface");
    println!("• Press 'I' for current state info");
    println!("• Press Space to reset stroke counter");
    println!("• Press Escape to exit");
    println!("• Mouse input works as fallback if no pen detected");
    println!("");
    println!("Expected pen data:");
    println!("• Position (x, y)");
    println!("• Pressure (0.0 - 1.0)");
    println!("• Tilt angles in degrees");
    println!("• Distance from surface");
    println!("• Rotation/twist angle");
    println!("• Tool type (Pen, Eraser, etc.)");
    println!("");

    miniquad::start(conf, || Box::new(Stage::new()));
}
