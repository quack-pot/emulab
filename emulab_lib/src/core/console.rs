pub trait Console {
    /** Run on console initial boot (power on state) */
    fn restart(&mut self);

    /**
     * General function for performing physical actions on
     * the console. (Reset button, Inserting or removing a
     * peripheral device, Etc.)
     */
    fn run_console_action(&mut self, action_code: u32);

    /** Runs the actions for the next frame's time. */
    fn advance_frame(&mut self);

    /** Grabs the current framebuffer as an RGBA image. */
    fn get_frame(&self) -> (Box<[u8]>, u32, u32);

    fn load_program(&mut self, filepath: String);
}
