#[derive(Default, Debug, Clone)]
pub struct Watch {
    // DBusWatch *watch;
    #[cfg(target_os = "linux")]
    pub watch: Vec<DBusWatch>,
    // struct watch *next;
}
