use FMFP::parse_input;

fn main() {
    let input = "\\x -> if ((snd x) 1) then \\y -> y else \\z -> (z +1)";

    let _parsed = parse_input(input);
}
