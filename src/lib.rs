/// Executes the code in the try block. If it panics, attempts to match the
/// type and pattern of the panic to an arm in the catch block. Each arm takes
/// the form `type, pattern => expr`. If a catch arm panics, that panic
/// is the one which propagates up. If no catch arm matches the panic
/// payload, then the originally thrown panic propagates up the callstack
/// as if there was no `try`/`catch` block. The types of the try block
/// and each catch arm must agree.
/// # Notes
/// This *only* catches unwinding panics.
/// # Examples
/// ```
/// use rsexcept::rsexcept;
/// fn modulo(a: u32, b: u32) -> u32 {
///     if b == 0 {
///         panic!("b was zero")
///     };
///     a % b
/// }
///
/// fn main() {
///     let res = rsexcept! {
///         try {
///             modulo(5, 2)
///         }
///         catch {
///             &str, s => {
///                 println!("{}", s);
///                 0
///             }
///         }
///     };
///     assert_eq!(1, res);
///     let res = rsexcept! {
///         try {
///             modulo(5, 0)
///         }
///         catch {
///             &str, s => {
///                 println!("{}", s);
///                 0
///             }
///         }
///     };
///     assert_eq!(0, res);
/// }
/// ```
/// ```
/// use rsexcept::rsexcept;
/// fn main() {
///     static ARR: [&'static str; 5] = ["hey", "this", "is", "a", "array"];
///     let res = rsexcept! {
///         try {
///             std::panic::panic_any(&ARR[1..]);
///         }
///         catch {
///             i32, _ => panic!("Nope!"),
///             &[&str], ["uh", s, ..] => s.to_string(),
///             &[&str], ["this", h, t @ ..] => {
///                 format!("{}_{}", h, t[1])
///             }
///         }
///     };
///     assert_eq!("is_array", res);
/// }
/// ```
#[macro_export]
macro_rules! rsexcept {
    (try $b:block catch { $( $t:ty, $p:pat => $handler:expr),* $(,)? }) => {
        {
            let old_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(|_| {}));
            match std::panic::catch_unwind(|| $b) {
                Ok(v) => v,
                Err(e) => {
                    std::panic::set_hook(old_hook);
                    $(
                        if let Some($p) = e.downcast_ref::<$t>(){
                            $handler
                        }
                        else
                    )*
                    {
                        std::panic::resume_unwind(e)
                    }
                }
            }
        }
    };
}

#[cfg(test)]
#[allow(unreachable_code)]
mod tests {
    use std::panic::panic_any;
    #[test]
    #[should_panic]
    fn empty() {
        rsexcept! {
            try {
                panic_any("Nothing should catch this")
            }
            catch {

            }
        }
    }
    #[test]
    fn no_panic() {
        let res = rsexcept! {
            try {
                86
            }
            catch {
                i32, i => i + 12
            }
        };
        assert_eq!(res, 86);
    }
    #[test]
    fn one_arm() {
        let res = rsexcept! {
            try {
                panic_any("Catch me if you can");
                52
            }
            catch {
                &str, _ => 42
            }
        };
        assert_eq!(42, res);
    }
    #[test]
    fn multi_arm() {
        let res = rsexcept! {
            try {
                panic_any(6.54);
                52
            }
            catch {
                i32, _ => panic!("Nope!"),
                &f64, _ => 65,
                f64, _ => 42,
                &str, _ => 87
            }
        };
        assert_eq!(42, res);
    }
    #[test]
    fn multi_arm_comma() {
        let res = rsexcept! {
            try {
                panic_any(6.54);
                52
            }
            catch {
                i32, _ => panic!("Nope!"),
                &f64, _ => 65,
                f64, _ => 42,
                &str, _ => 87, // terminal comma
            }
        };
        assert_eq!(42, res);
    }
    #[test]
    fn patterns() {
        static ARR: [&'static str; 5] = ["hey", "this", "is", "a", "array"];
        let res = rsexcept! {
            try {
                panic_any(&ARR[1..]);
                "Uh-oh, somersault jump!".to_string()
            }
            catch {
                i32, _ => panic!("Nope!"),
                &str, s => s.to_string(),
                &[&str], ["uh", s, ..] => s.to_string(),
                &[&str], ["this", h, t @ ..] => {
                    format!("{}_{}", h, t[1])
                }
            }
        };
        assert_eq!("is_array", res);
    }
    #[test]
    fn block() {
        let res = rsexcept! {
            try {
                panic_any(62);
                "62".to_string()
            }
            catch {
                i32, i => {
                    let i = i + 2;
                    i.to_string()
                }
            }
        };
        assert_eq!("64", res);
    }
    #[test]
    #[should_panic]
    fn panic_in_handler() {
        rsexcept! {
            try {
                panic_any(62);
                ()
            }
            catch {
                i32, _ => {
                    panic!("Catch me")
                }
            }
        };
    }
    #[test]
    fn panic_propagate() {
        let res = rsexcept! {
            try {
                rsexcept! {
                    try {
                        panic_any(62)
                    }
                    catch {
                        i32, _ => {
                            panic!("Catch me")
                        }
                    }
                }
            }
            catch {
                &str, s => format!("\"{}\"? Caught you!", s)
            }
        };
        assert_eq!(res, "\"Catch me\"? Caught you!");
    }
}
