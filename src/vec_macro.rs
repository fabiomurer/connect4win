#[macro_export]
macro_rules! vec_macro {
    ($value:expr, $size:expr) => {
        ([$value; $size as usize], 0 as usize, $size as usize)
    };
}

#[macro_export]
macro_rules! push {
    ($v:expr, $val:expr) => {
        $v.0[$v.1] = $val;
        $v.1 += 1;
    };
}

#[macro_export]
macro_rules! pop {
    ($v:expr) => {{
        $v.1 -= 1;
        let n = $v.0[$v.1];
        n
    }};
}

#[macro_export]
macro_rules! get_array {
    ($v:expr) => {
        &$v.0[0..$v.1]
    };
}

#[macro_export]
macro_rules! len {
    ($vec:expr) => {
        $vec.1
    };
}

#[macro_export]
macro_rules! max_len {
    ($vec:expr) => {
        $vec.2
    };
}

#[macro_export]
macro_rules! showtype {
    ($t:ty, $size:expr) => {
        ([$t; $size], usize, usize)
    };
}

#[macro_export]
macro_rules! show {
    ($v:expr) => {
        println!(
            "array: {:?}, len {}, max len {}",
            &$v.0[0..$v.1],
            $v.1,
            $v.2
        )
    };
}
#[cfg(test)]
mod tests {
    #[test]
    fn sas() {
        let mut sis = vec_macro!(0 as i32, 12);
        let arr = [0; 12];
        assert_eq!(arr.len(), max_len!(sis));
        push!(sis, 3);
        push!(sis, 2);
        show!(sis);
        let tre = pop!(sis);
        assert_eq!(2, tre);
        assert_eq!(1, len!(sis))
    }
}
