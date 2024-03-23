import { Center, Code, Text, Textarea } from "@mantine/core";
import { MathJax } from "better-react-mathjax";
import Card from "./Card";

type ConstraintsProps = {
  constraints: string[];
};

const Constraints = ({ constraints }: ConstraintsProps) => {
  return (
    <Card title="Constraints">
      {constraints.length === 0 && <Text c="red.2">No constraints</Text>}

      {constraints.map((constraint, index) => (
        <MathJax key={index}>{"\\(" + constraint + " \\)"}</MathJax>
      ))}
    </Card>
  );
};

export default Constraints;
