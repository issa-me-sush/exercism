// Export the macro
#[macro_export]
macro_rules! hashmap {
    // Handle the case with no arguments
    () => {
        ::std::collections::HashMap::new()
    };
    // Handle the case with a single comma
    (,) => {
        compile_error!("Expected key-value pairs separated by commas, but found only a comma.")
    };
    // Handle the case with one or more key-value pairs
    ($( $key:expr => $value:expr ),* $(,)?) => {
        {
            let mut hm = ::std::collections::HashMap::new();
            $(
                hm.insert($key, $value);
            )*
            hm
        }
    };
}
