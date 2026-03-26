#[macro_export]
macro_rules! init {
    ($global:ident, $value:expr) => {
        let _ = $global.with(|inner| inner.set($value));
    };
}

#[macro_export]
macro_rules! util {
    ($global:ident, $value:ident, $body:block) => {
        $global.with(|inner| {
            let $value = inner.get().unwrap();
            $body
        })
    };
}

#[macro_export]
macro_rules! call {
    ($global:ident, $($arg:expr),* $(,)?) => {
        $global.with(|inner| (inner.get().unwrap())($($arg),*))
    };
}