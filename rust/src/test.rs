use FMFP::parse_input;

fn main() {
    let input = "\\x -> if ((snd x) 1) then \\y -> y else \\z -> (z +1)";

    let parsed = parse_input(input);
}
