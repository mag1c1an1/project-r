use tklog::LOG;

pub fn init_log() {
    LOG.set_console(true)
        .set_cutmode_by_size("rnemu.log", 10000, 2, false);
}
