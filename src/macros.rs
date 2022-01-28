macro_rules! pextln {
    () => ($macros::pextln!(""));
    ($arg:literal) => ({
        println!(concat!("[T7-Tool-Ext] (", env!("CARGO_PKG_VERSION"), ") ", $arg));
    });
    ($($arg:tt)*)  => ({
        println!(concat!("[T7-Tool-Ext] (", env!("CARGO_PKG_VERSION"), ") {}"), format_args!($($arg)*));
    });
}
