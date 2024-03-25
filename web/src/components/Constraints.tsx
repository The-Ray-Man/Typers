import { Center, Code, SimpleGrid, Stack, Text, Textarea } from "@mantine/core";
import { MathJax } from "better-react-mathjax";
import Card from "./Card";

type ConstraintsProps = {
  constraints: string[];
  constraints_without_trivial: string[];
};

const Constraints = ({
  constraints,
  constraints_without_trivial,
}: ConstraintsProps) => {
  return (
    <Card title="Constraints">
      {constraints.length === 0 && <Text c="red.2">No constraints</Text>}

      <SimpleGrid cols={2} w={"100%"}>
        <Stack>
          <Center>
            <Text>Constraints</Text>
          </Center>
          {constraints.map((constraint, index) => {
            if (constraints_without_trivial.includes(constraint)) {
              return (
                <Center>
                  <MathJax key={index}>{"\\(" + constraint + " \\)"}</MathJax>
                </Center>
              );
            } else {
              return (
                <Center>
                  <MathJax key={index} style={{ color: "red" }}>
                    {"\\(" + constraint + " \\)"}
                  </MathJax>
                </Center>
              );
            }
          })}
        </Stack>
        <Stack>
          <Center>
            <Text>Constraints without trivial</Text>
          </Center>
          {constraints_without_trivial.map((constraint, index) => (
            <Center>
              <MathJax key={index}>{"\\(" + constraint + " \\)"}</MathJax>
            </Center>
          ))}
        </Stack>
      </SimpleGrid>
    </Card>
  );
};

export default Constraints;
