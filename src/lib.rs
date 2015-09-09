//! # pipe_macros
//! A small macro library that allows you to pipe functions
//! similar to the pipe operator in Elixir and F# (|>)

#![deny(missing_docs)]
#![deny(warnings)]

#[macro_export]
macro_rules! pipe_fun {
    (&, $ret:expr) => {
        &$ret;
    };
    ((as $typ:ty), $ret:expr) => {
        $ret as $typ;
    };
    ({$fun:expr}, $ret:expr) => {
        $fun($ret);
    };
    ([$fun:ident], $ret:expr) => {
        $ret.$fun();
    };
    (($fun:ident($($arg:expr),*)), $ret:expr) => {
        $fun($ret $(,$arg)*);
    };
    ($fun:ident, $ret:expr) => {
        $fun($ret);
    }
}

#[macro_export]
macro_rules! pipe {
    ( $expr:expr => $($funs:tt)=>+ ) => {
        {
            let ret = $expr;
            $(
                let ret = pipe_fun!($funs, ret);
            )*
            ret
        }
    };
}

#[macro_export]
macro_rules! pipe_res {
    ( $expr:expr => $($funs:tt)=>+ ) => {
        {
            let ret = Ok($expr);
            $(
                let ret = match ret {
                    Ok(x) => pipe_fun!($funs, x),
                    _ => ret
                };
            )*
            ret
        }
    };
}

#[macro_export]
macro_rules! pipe_opt {
    ( $expr:expr => $($funs:tt)=>+ ) => {
        {
            let ret = None;
            $(
                let ret = match ret {
                    None => pipe_fun!($funs, $expr),
                    _ => ret
                };
            )*
            ret
        }
    };
}

#[cfg(test)]
mod test_pipe_opt{
    fn times2(a: u32) -> Option<u32>{
        return Some(a * 2);
    }

    fn nope(_a: u32) -> Option<u32>{
        return None;
    }

    #[test]
    fn accepts_options() {
        let ret = pipe_opt!(
            4
            => times2
        );

        assert_eq!(ret, Some(8));
    }

    #[test]
    fn accepts_unwrap() {
        let ret = pipe_opt!(
            4
            => times2
        ).unwrap();

        assert_eq!(ret, 8);
    }


    #[test]
    fn exits_early() {
        let ret = pipe_opt!(
            4
            => times2
            => times2
            => times2
        );

        assert_eq!(ret, Some(8));
    }

    #[test]
    fn goes_until_some() {
        let ret = pipe_opt!(
            4
            => nope
            => nope
            => {|_i: u32| None}
            => times2
            => nope
        );

        assert_eq!(ret, Some(8));
    }

    #[test]
    fn ends_with_none() {
        let ret = pipe_opt!(
            4
            => nope
            => nope
            => {|_i: u32| None}
            => nope
        );

        assert_eq!(ret, None);
    }
}


#[cfg(test)]
mod test_pipe_res{
    fn times2(a: u32) -> Result<u32, String>{
        return Ok(a * 2);
    }

    fn fail_if_over_4(a: u32) -> Result<u32, String>{
        if a > 4 {
            return Err("This number is larger than four".to_string());
        }
        return Ok(a);
    }

    #[test]
    fn accepts_results() {
        let ret = pipe_res!(
            4
            => times2
        );

        assert_eq!(ret, Ok(8));
    }

    #[test]
    fn accepts_unwrap() {
        let ret = pipe_res!(
            4
            => times2
        ).unwrap();

        assert_eq!(ret, 8);
    }


    #[test]
    fn chains_result_values() {
        let ret = pipe_res!(
            4
            => times2
            => times2
            => times2
        );

        assert_eq!(ret, Ok(32));
    }

    #[test]
    fn exits_early() {
        let ret = pipe_res!(
            4
            => times2
            => fail_if_over_4
            => times2
            => times2
        );

        assert_eq!(ret, Err("This number is larger than four".to_string()));
    }
}

#[cfg(test)]
mod test_pipe{
    fn times2(a: u32) -> u32{
        return a * 2;
    }

    fn times(a: u32, b: u32, c: u32) -> u32{
        return a * b * c;
    }

    #[test]
    fn test_int() {
        let multiply = |i: u32| i * 2;
        let ret = pipe!(
            4
            => times2
            => {|i: u32| i * 2}
            => multiply
            => (times(100, 10))
        );

        assert_eq!(ret, 32000);
    }

    #[test]
    fn test_string() {
        let ret = pipe!(
            "abcd"
            => [len]
            => (as u32)
            => times2
            => (times(100, 10))
            => [to_string]
        );

        //let ret = "abcd";
        //let ret = ret.len();
        //let ret = ret as u32;
        //let ret = times2(ret);
        //let ret = times(ret, 100, 10);
        //let ret = ret.to_string();

        assert_eq!(ret, times(times2(("abcd".len() as u32)), 100, 10).to_string());
        assert_eq!(ret, "8000");
    }
}

