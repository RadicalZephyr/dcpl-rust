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
