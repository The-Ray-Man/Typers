import { Center, Flex, SimpleGrid, Stack, Text, Title } from "@mantine/core";
import { ResultAccumulateStepTS } from "FMFP";
import { MathJax } from "better-react-mathjax";

type AccumulateStepProps = {
  step: ResultAccumulateStepTS;
};

const AccumulateStep = ({ step }: AccumulateStepProps) => {
  return (
    <Stack w={"100%"}>
      <Title order={3}>Accumulate Step</Title>

      <SimpleGrid cols={3} w={"100%"}>
        <Stack>
          <Text>Old Rules</Text>
          {step.rules_before.map((rule) => {
            if (step.rules_compared.includes(rule)) {
              return (
                <MathJax style={{ color: "blue" }}>
                  {"\\(" + rule + "\\)"}
                </MathJax>
              );
            } else {
              return <MathJax>{"\\(" + rule + "\\)"}</MathJax>;
            }
          })}
        </Stack>
        <Stack>
          <Text>Compared Rules</Text>
          {step.rules_compared.map((rule) => (
            <MathJax style={{ color: "blue" }}>{"\\(" + rule + "\\)"}</MathJax>
          ))}
          <Text>New Rules</Text>
          {step.rules_added.length === 0 && (
            <Text c={"dimmed"}>
              This Accumumulation did not yield any new rules
            </Text>
          )}
          {step.rules_added.map((rule) => (
            <MathJax style={{ color: "green" }}>{"\\(" + rule + "\\)"}</MathJax>
          ))}
        </Stack>
        <Stack>
          <Text>Next Rules</Text>
          {step.rules_after.map((rule) => {
            if (step.rules_added.includes(rule)) {
              return (
                <MathJax style={{ color: "green" }}>
                  {"\\(" + rule + "\\)"}
                </MathJax>
              );
            } else if (step.rules_compared.includes(rule)) {
              return (
                <MathJax style={{ color: "blue" }}>
                  {"\\(" + rule + "\\)"}
                </MathJax>
              );
            } else {
              return <MathJax>{"\\(" + rule + "\\)"}</MathJax>;
            }
          })}
        </Stack>
      </SimpleGrid>
    </Stack>
  );
};

export default AccumulateStep;
