use std::str::FromStr;

pub fn uints<F: FromStr>(s: &str) -> impl Iterator<Item = F> {
    let non_digit = |c: char| !c.is_digit(10);
    s.split(non_digit).filter_map(|x| x.parse::<F>().ok())
}

pub fn ints<F: FromStr>(s: &str) -> impl Iterator<Item = F> {
    let is_not_int = |c: char| !(c.is_digit(10) || c == '-');
    s.split(is_not_int).filter_map(|x| x.parse::<F>().ok())
}
