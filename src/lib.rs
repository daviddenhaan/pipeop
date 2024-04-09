#[macro_export]
macro_rules! pipe {
    (@accumulate_individual_pipes [$($callback:tt)*] [] |> $($pipes:tt)+) => ($crate::pipe!(
        @accumulate_individual_pipes [$($callback)*] [] [] $($pipes)+
    ));

    (@accumulate_individual_pipes [$($callback:tt)*] [$([ $($pipe:tt)* ])*] [$($buffer:tt)*] |> $($pipes:tt)+) => ($crate::pipe!(
        @accumulate_individual_pipes [$($callback)*] [$([ $($pipe)* ])* [$($buffer)*]] [] $($pipes)+
    ));

    (@accumulate_individual_pipes [$($callback:tt)*] [$([ $($pipe:tt)* ])*] [$($buffer:tt)*] $tt:tt $($tail:tt)*) => ($crate::pipe!(
        @accumulate_individual_pipes [$($callback)*] [$([ $($pipe)* ])*] [$($buffer)* $tt] $($tail)*
    ));

    (@accumulate_individual_pipes [$($callback:tt)*] [$([ $($pipe:tt)* ])*] [$($buffer:tt)*]) => ($crate::pipe!(
        $($callback)* [$([ $($pipe)* ])* [$($buffer)*]]
    ));

    (@accumulate_individual_pipes [$($callback:tt)*] $($pipes:tt)+) => ($crate::pipe!(
        @accumulate_individual_pipes [$($callback)*] [] $($pipes)+
    ));

    (@accumulated_expr [$expr:expr] $($pipes:tt)+) => ($crate::pipe!(
        @accumulate_individual_pipes [@make_pipeline [$expr]] $($pipes)*
    ));

    (@accumulate_expression [$($callback:tt)*] [$($buffer:tt)*] $tt:tt |> $($pipes:tt)+) => ($crate::pipe!(
        @finish_expression [$($callback)*] [$($buffer)* $tt] |> $($pipes)+
    ));

    (@accumulate_expression [$($callback:tt)*] [$($buffer:tt)*] $tt:tt $($tail:tt)+) => ($crate::pipe!(
        @accumulate_expression [$($callback)*] [$($buffer)* $tt] $($tail)+
    ));

    (@accumulate_expression [$($callback:tt)*] $($tokens:tt)+) => ($crate::pipe!(
        @accumulate_expression [$($callback)*] [] $($tokens)+
    ));

    (@finish_expression [$($callback:tt)*] [$expr:expr] $($pipes:tt)+) => ($crate::pipe!(
        $($callback)* [$expr] $($pipes)*
    ));

    (@finish_expression [$($callback:tt)*] [$($false_expr:tt)*] $($residual:tt)*) => (::std::compile_error!(
        "could not accumulate an expression for use in the pipeline"
    ));

    (@make_pipeline [$expr:expr] [$([ $($pipe:tt)+ ])+]) => ({
        let current = $expr;
        $(
            macro_rules! __pipeop_expand_to_current {() => (current)}
            let current = $crate::pipe!(@transform_pipe $($pipe)+);
        )+
        current
    });

    (@make_pipeline $($tokens:tt)+) => ($crate::pipe!(
        @accumulate_expression [@accumulated_expr] $($tokens)*
    ));

    (@transform_pipe $(<$ty:ty>)? . $($method:tt)+) => ($crate::pipe!(
        @maybe_ends_with_try [@transform_pipe] [] |item $(: $ty)?| item.$($method)+
    ));

    (@transform_pipe [$(try $(@$($_:tt)* $has_try:tt)?)?] $($pipe:tt)*) => ($crate::pipe!(
        @finalize_pipe [ $($($has_try)? [try])? ] $($pipe)*
    ));

    (@transform_pipe $expr:expr) => ($crate::pipe!(@finalize_pipe [] |item| $expr(item)));

    (@maybe_ends_with_try [$($callback:tt)*] [$($buffer:tt)*] ?) => ($crate::pipe!($($callback)* [try] $($buffer)*));

    (@maybe_ends_with_try [$($callback:tt)*] [$($buffer:tt)*]) => ($crate::pipe!($($callback)* [] $($buffer)*));

    (@maybe_ends_with_try [$($callback:tt)*] [$($buffer:tt)*] $tt:tt $($tail:tt)*) => ($crate::pipe!(
        @maybe_ends_with_try [$($callback)*] [$($buffer)* $tt] $($tail)*
    ));

    (@finalize_pipe [$([ $modifier:tt ])*] $($pipe:tt)+) => ($crate::pipe!(
        @apply_modifiers [$([ $modifier ])*] [$crate::call_with($($pipe)+, __pipeop_expand_to_current!())]
    ));

    (@apply_modifiers [[try] $($modifiers:tt)*] [$($pipe:tt)*]) => ($crate::pipe!(@apply_modifiers [$($modifiers)*] [$($pipe)*?]));

    (@apply_modifiers [[$($unknown:tt)+] $($_:tt)*]) => (
        ::std::compile_error!(::std::concat!(
            "unknown modifier: ",
            ::std::stringify!($($unknown)*)),
        )
    );

    (@apply_modifiers [] [$($pipe:tt)*]) => ($($pipe)*);

    ($($tokens:tt)+) => ($crate::pipe!(
        @make_pipeline $($tokens)+
    ));

    // An empty pipeline results in a unit type.
    () => (());
}

pub fn call_with<T, R, F: FnOnce(T) -> R>(f: F, t: T) -> R {
    f(t)
}

#[cfg(test)]
mod tests {
    use std::num::ParseIntError;
    use std::ops::Add;

    #[test]
    fn fn_pipe() {
        const fn add_one(to: i32) -> i32 {
            to + 1
        }

        let result = pipe!(1 |> add_one);
        assert_eq!(result, 2);
    }

    #[test]
    fn closure_pipe() {
        let result = pipe!(1 |> |item| item + 1);
        assert_eq!(result, 2);
    }

    #[test]
    fn method_invocation_pipe() {
        let result = pipe!(1 |> .add(1));
        assert_eq!(result, 2);
    }

    #[test]
    fn explicit_type_expectation_in_method_invocation() {
        let result = pipe!(1 |> <i32>.add(2) |> <i32>.add(1));
        assert_eq!(result, 4);
    }

    #[test]
    fn many_pipes() {
        let result = pipe!(1i32
            |> .add(1)
            |> |item| item + 1
            |> <i32>.add(1)
        );

        assert_eq!(result, 4);
    }

    #[test]
    fn can_use_try_modifier() -> Result<(), ParseIntError> {
        let result = pipe!("1" |> .parse::<u8>()?);
        assert_eq!(result, 1u8);
        Ok(())
    }
}
