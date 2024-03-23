import { Center, Code, Stack, Text, Textarea } from "@mantine/core";
import { MathJax } from "better-react-mathjax";

import Card from "./Card";

type SolutionProps = {
  solution: string;
};

const Solution = ({solution} : SolutionProps) => {
    return (
        <Card title="Solution">
            <MathJax>
                {"\\(" + solution +"\\)"}
       </MathJax>
        </Card>
    );
    }

export default Solution;