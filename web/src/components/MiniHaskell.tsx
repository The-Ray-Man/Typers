import { Flex } from "@mantine/core";
import { MathJax } from "better-react-mathjax";

const MiniHaskell = () => {
  return (
    <Flex justify={"center"} gap={"md"} wrap={"wrap"} maw={600}>
      <MathJax>
        {
          "\\(\\dfrac{ } {\\dots, x \\: : \\: \\tau, \\dots \\vdash {x \\: :: \\: \\tau}} \\textsf{Var}\\)"
        }
      </MathJax>
      <MathJax>
        {
          "\\(\\dfrac{ \\Gamma, x \\: : \\sigma \\vdash t \\: :: \\: \\tau} {\\Gamma \\vdash {\\lambda x.t \\: :: \\sigma\\to \\: \\tau}} \\textsf{Abs}\\)"
        }
      </MathJax>

      <MathJax>
        {
          "\\(\\dfrac{ \\Gamma \\vdash t_1 \\: :: \\sigma \\to  \\tau \\qquad \\Gamma \\vdash t_2 :: \\sigma} {\\Gamma \\vdash {t_1 \\: t_2 :: \\tau}} \\textsf{App}\\)"
        }
      </MathJax>

      <MathJax>
        {
          "\\(\\dfrac{\\Gamma \\vdash t :: \\textit{Int} } {\\Gamma \\vdash {\\textbf{iszero}\\: t  :: \\textit{Bool}}} \\textsf{iszero}\\)"
        }
      </MathJax>
      <MathJax>
        {
          "\\(\\dfrac{ } {\\Gamma \\vdash {\\textit{n}  :: \\textit{Int}}} \\textsf{Int}\\)"
        }
      </MathJax>
      <MathJax>
        {
          "\\(\\dfrac{ } {\\Gamma \\vdash {\\textit{true}  :: \\textit{Bool}}} \\textsf{True}\\)"
        }
      </MathJax>

      <MathJax>
        {
          "\\(\\dfrac{ } {\\Gamma \\vdash {\\textit{false}  :: \\textit{Bool}}} \\textsf{False}\\)"
        }
      </MathJax>

      <MathJax>
        {
          "\\(\\dfrac{ \\Gamma \\vdash t_1 \\: :: \\textit{Int} \\qquad \\Gamma \\vdash t_2 :: \\textit{Int} } {\\Gamma \\vdash {t_1\\: \\textbf{op}\\: t_2 :: \\textit{Int} }} \\textsf{BinOp} \\qquad \\textsf{for } \\textbf{op} \\in \\{+,*\\}\\)"
        }
      </MathJax>
      <MathJax>
        {
          "\\(\\dfrac{ \\Gamma \\vdash t_0 \\: :: \\textit{Bool} \\qquad \\Gamma \\vdash t_1 :: \\tau \\qquad \\Gamma \\vdash t_2 :: \\tau } {\\Gamma \\vdash {\\textbf{if} \\: t_o \\: \\textbf{then} \\: t_1 \\: \\textbf{else} \\: t_2 :: \\tau}} \\textsf{if}\\)"
        }
      </MathJax>
      <MathJax>
        {
          "\\(\\dfrac{ \\Gamma \\vdash t_1 :: \\tau_1 \\qquad \\Gamma \\vdash t_2 :: \\tau_2 } {\\Gamma \\vdash {(t_1,t_2)::(\\tau_1, \\tau_2)}} \\textsf{Tuple}\\)"
        }
      </MathJax>
      <MathJax>
        {
          "\\(\\dfrac{ \\Gamma \\vdash t_1 :: (\\tau_1, \\tau_2) } {\\Gamma \\vdash {\\textbf{fst} \\: t :: \\tau_1}} \\textsf{fst}\\)"
        }
      </MathJax>
      <MathJax>
        {
          "\\(\\dfrac{ \\Gamma \\vdash t_1 :: (\\tau_1, \\tau_2) } {\\Gamma \\vdash {\\textbf{snd} \\: t :: \\tau_2}} \\textsf{snd}\\)"
        }
      </MathJax>
    </Flex>
  );
};

export default MiniHaskell;
