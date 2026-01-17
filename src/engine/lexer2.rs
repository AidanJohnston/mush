trait Token {
    const NEXT: &'static [&dyn Token];
}
