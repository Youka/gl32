#[cfg(test)]
mod gl32_tests {
    // Imports
    use glutin::*;
    use glutin::dpi::*;

    #[test]
    fn test_gl_window() {
        // Create OpenGL window
        let mut events_loop = EventsLoop::new();
        let gl_window = GlWindow::new(
            WindowBuilder::new()
                .with_title("GL32 test window!")
                .with_dimensions(LogicalSize::new(1280.0, 720.0)),
            ContextBuilder::new()
                .with_vsync(true),
            &events_loop
        ).expect("Couldn't create simple GL window!");

        // Initialize context (activate context & load modern GL functions)
        unsafe {
            gl_window.make_current().expect("Couldn't make GL context current!");
        }
        let gl = gl32::Gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

        // Run event loop
        events_loop.run_forever(|event| {
            // Window event?
            if let Event::WindowEvent { event, .. } = event {
                // Window closed
                if let WindowEvent::CloseRequested = event {
                    return ControlFlow::Break;
                }
            }

            // Draw!
            unsafe {
                gl.ClearColor(0.0, 1.0, 0.0, 1.0);
                gl.Clear(gl32::COLOR_BUFFER_BIT);
            }

            // Update screen
            gl_window.swap_buffers().expect("Couldn't swap GL pixel buffers!");

            // Continue loop (not!)
            ControlFlow::Break
            //ControlFlow::Continue
        });
    }

    #[test]
    fn test_gl32_available() {
        GlWindow::new(
            // Minimal invisible window (at least required for offscreen rendering)
            WindowBuilder::new()
                .with_dimensions(LogicalSize::new(1.0, 1.0))
                .with_visibility(false),
            // Request powerful enough OpenGL profile
            ContextBuilder::new()
                .with_gl(GlRequest::Specific(Api::OpenGl, (3, 2)))
                .with_gl_profile(GlProfile::Core),
            // Just required for window building
            &EventsLoop::new()
        ).expect("Couldn't initialize simple window with GL requirements!");
    }
}