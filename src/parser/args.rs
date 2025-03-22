pub fn retrieve_args(input: &str) -> Option<(&str, &str)> {
    let mut args = input.trim().split('=');
    let cmd = args.next()?;
    let arg = args.next()?;

    if args.next().is_some() {
        return None;
    }

    Some((cmd, arg))
}
