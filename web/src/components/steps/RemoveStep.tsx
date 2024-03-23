import { Center, Flex, SimpleGrid, Stack, Text, Title } from "@mantine/core";
import { IconArrowRight } from "@tabler/icons-react";
import { ResultRemoveStepTS } from "FMFP";
import { MathJax } from "better-react-mathjax";

type RemoveStepProps = {
  step: ResultRemoveStepTS;
};

const RemoveStep = ({ step }: RemoveStepProps) => {
  return (
    <Stack w={"100%"}>
      <Title order={3}>Remove Step</Title>

      <MathJax>{step.text}</MathJax>

      <SimpleGrid cols={3} w={"100%"}>
        <Stack>
          {step.rules_before.map((rule) => {
            if (step.rules_removed.includes(rule)) {
              return (
                <MathJax style={{ color: "red" }}>
                  {"\\(" + rule + "\\)"}
                </MathJax>
              );
            } else {
              return <MathJax>{"\\(" + rule + "\\)"}</MathJax>;
            }
          })}
        </Stack>
        <Stack>
          {step.rules_after.map((rule) => {
            if (step.rules_before.includes(rule)) {
              return <MathJax>{"\\(" + rule + "\\)"}</MathJax>;
            } else {
              return (
                <MathJax style={{ color: "blue" }}>
                  {"\\(" + rule + "\\)"}
                </MathJax>
              );
            }
          })}
        </Stack>
      </SimpleGrid>
    </Stack>
  );
};

export default RemoveStep;
