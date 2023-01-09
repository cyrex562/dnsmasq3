#[derive(Default, Debug, Clone)]
pub struct watch {
    // DBusWatch *watch;
    pub watch: *mut DBusWatch,
    // struct watch *next;
}
