#[macro_export]
macro_rules! arith_op {
    { $stack:ident, $op:tt } => {{
        let v1 = $stack
            .pop()
            .ok_or(Error::NotEnoughValues)?
            .into_integer()
            .ok_or(Error::NotANumber)?;
        let v2 = $stack
            .pop()
            .ok_or(Error::NotEnoughValues)?
            .into_integer()
            .ok_or(Error::NotANumber)?;

        $stack.push(StackValue::Integer(v2 $op v1));
        Ok($stack)
    }};
}

#[macro_export]
macro_rules! bool_op {
    { $stack:ident, $op:tt } => {{
        let v1 = $stack
            .pop()
            .ok_or(Error::NotEnoughValues)?
            .into_integer()
            .ok_or(Error::NotANumber)?;
        let v2 = $stack
            .pop()
            .ok_or(Error::NotEnoughValues)?
            .into_integer()
            .ok_or(Error::NotANumber)?;

        $stack.push(StackValue::Integer(if v2 $op v1 {1} else {0}));
        Ok($stack)
    }};
}

#[macro_export]
macro_rules! stack {
    { $($val:expr),* }=> {{
        let v = vec![ $($val),* ];
        v.into_iter().map(StackValue::from).collect::<Vec<StackValue>>()
    }}
}

#[macro_export]
macro_rules! arith_op_test {
    { $name:ident : $operator:expr => [ $($stack_val:expr),* ] == $expected:expr } => {
        #[test]
        fn $name() {
            assert_eq!(Ok(stack![$expected]), Program::apply_builtin(stack![ $($stack_val),* ], &$operator));
        }
    }
}

#[macro_export]
macro_rules! bool_op_test {
    { $name:ident : $operator:expr => [ $($stack_val:expr),* ] -> true } => {
        #[test]
        fn $name() {
            assert_eq!(Ok(stack![1]), Program::apply_builtin(stack![ $($stack_val),* ], &$operator));
        }
    };
    { $name:ident : $operator:expr => [ $($stack_val:expr),* ] -> false } => {
        #[test]
        fn $name() {
            assert_eq!(Ok(stack![0]), Program::apply_builtin(stack![ $($stack_val),* ], &$operator));
        }
    };
}
