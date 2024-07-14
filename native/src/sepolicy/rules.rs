use crate::{ffi::Xperm, sepolicy, SepolicyMagisk};
use base::{set_log_level_state, LogLevel};
use std::pin::Pin;

macro_rules! rules {
    (@args all) => {
        vec![]
    };
    (@args xall) => {
        vec![Xperm { low: 0x0000, high: 0xFFFF, reset: false }]
    };
    (@args svcmgr) => {
        vec!["servicemanager", "vndservicemanager", "hwservicemanager"]
    };
    (@args [proc]) => {
        vec!["magisk"]
    };
    (@args [file]) => {
        vec!["magisk_file"]
    };
    (@args [log]) => {
        vec!["magisk_log_file"]
    };
    (@args proc) => {
        "magisk"
    };
    (@args file) => {
        "magisk_file"
    };
    (@args log) => {
        "magisk_log_file"
    };
    (@args [$($arg:tt)*]) => {
        vec![$($arg)*]
    };
    (@args $arg:expr) => {
        $arg
    };
    (@stmt $self:ident) => {};
    (@stmt $self:ident $action:ident($($args:tt),*); $($res:tt)*) => {
        $self.as_mut().$action($(rules!(@args $args)),*);
        rules!{@stmt $self $($res)* }
    };
    (use $self:ident; $($res:tt)*) => {{
        rules!{@stmt $self $($res)* }
    }};
}

impl SepolicyMagisk for sepolicy {
    fn magisk_rules(mut self: Pin<&mut Self>) {
        // Temp suppress warnings
        set_log_level_state(LogLevel::Warn, false);
        rules! {
            use self;
            type_(proc, ["domain"]);
            typeattribute([proc], ["mlstrustedsubject", "netdomain", "appdomain"]);
            type_(file, ["file_type"]);
            typeattribute([file], ["mlstrustedobject"]);

            // Make our root domain unconstrained
            allow([proc], [
                "fs_type", "dev_type", "file_type", "domain",
                "service_manager_type", "hwservice_manager_type", "vndservice_manager_type",
                "port_type", "node_type", "property_type"
            ], all, all);

            // Just in case, make the domain permissive
            permissive([proc]);

            // Allow us to do any ioctl
            allowxperm([proc], ["fs_type", "dev_type", "file_type", "domain"],
                ["blk_file", "fifo_file", "chr_file", "file"], xall);
            allowxperm([proc], [proc], ["tcp_socket", "udp_socket", "rawip_socket"], xall);

            // Let binder work with our processes
            allow(svcmgr, [proc], ["dir"], ["search"]);
            allow(svcmgr, [proc], ["file"], ["open", "read", "map"]);
            allow(svcmgr, [proc], ["process"], ["getattr"]);
            allow(["domain"], [proc], ["binder"], ["call", "transfer"]);

            // Other common IPC
            allow(["domain"], [proc], ["process"], ["sigchld"]);
            allow(["domain"], [proc], ["fd"], ["use"]);
            allow(["domain"], [proc], ["fifo_file"], ["write", "read", "open", "getattr"]);

            // For mounting loop devices, mirrors, tmpfs
            allow(["kernel"], ["fs_type", "dev_type", "file_type"], ["file"], ["read", "write"]);
        }

        #[cfg(any())]
        self.as_mut().strip_dontaudit();

        set_log_level_state(LogLevel::Warn, true);
    }
}
