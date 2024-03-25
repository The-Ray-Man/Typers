use fmfp::solve;

fn main() {
    let input = "\\x -> if ((snd x) 1) then \\y -> ((fst x) y) else \\z -> (iszero ((z +1) * 3))";

    let _parsed = solve(input);
}
