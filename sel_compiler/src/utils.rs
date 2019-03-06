pub fn loop_max<T>(max: usize, mut f: T)
where
    T: FnMut(),
{
    let mut loop_count = 0;
    loop {
        f();

        // fail safe
        // iterate maximum of nodes length
        if loop_count > max {
            break;
        }

        loop_count += 1;
    }
}
