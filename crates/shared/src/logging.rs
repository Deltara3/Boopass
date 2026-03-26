#[macro_export]
macro_rules! info {
    ($name:literal, $($arg:tt)*) => {
        println!(
            "[\x1B[1;92mINFO\x1B[0m] [\x1B[94m{}\x1B[0m] {}",
            $name,
            format_args!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! warn {
    ($name:literal, $($arg:tt)*) => {
        println!(
            "[\x1B[1;93mWARN\x1B[0m] [\x1B[94m{}\x1B[0m] {}",
            $name,
            format_args!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! fatal {
    ($name:literal, $($arg:tt)*) => {
        println!(
            "[\x1B[1;91mFATAL\x1B[0m] [\x1B[94m{}\x1B[0m] {}",
            $name,
            format_args!($($arg)*)
        );
    };
}