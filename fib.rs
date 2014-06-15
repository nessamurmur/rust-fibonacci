fn fib(num: int) -> int {

    fn _fib(num: int, acc: int, modifier: int) -> int {
        match(num, acc, modifier) {
            (0, _, _) => acc,
            _         => _fib(num-1, acc + modifier, acc)
        }
    }
    _fib(num, 0, 1)
}

fn main() {
    for num in range(0, 11) {
        println!("{}", fib(num))
    }
}
