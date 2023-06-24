fn factorail(n: u32) -> u32 {
    if n == 0 {
        1
    }

    else {
        n * factorail(n - 1)
    }
}