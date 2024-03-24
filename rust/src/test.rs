use FMFP::solve;

fn main() {
    let input = "\\x -> if ((snd x) 1) then \\y -> y else \\z -> (z +1)";

    let _parsed = solve(input);
}
