#[macro_export]
macro_rules! pointerchain {
    ($result: ident, $( $x:expr ),* ) => {
        unsafe {
            let mut current = 0;
            $(
                current = *((current + ($x as usize)) as *const usize);
                if current == 0x0 {
                    return 0;
                }
            )*
            &*(current as *const $result)
        }
    };
}