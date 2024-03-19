use one::one;

pub fn four() -> usize {
    one() + 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
