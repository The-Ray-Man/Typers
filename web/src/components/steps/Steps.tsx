import {
  Box,
  Center,
  Code,
  Divider,
  Stack,
  Text,
  Textarea,
} from "@mantine/core";
import { MathJax } from "better-react-mathjax";

import Card from "../Card";
import { ResultAccumulateStepTS, ResultRemoveStepTS, SolutionTS } from "FMFP";
import { useState } from "react";
import { ResultSubstituteStepTS } from "FMFP";
import AccumulateStep from "./AccumulateStep";
import RemoveStep from "./RemoveStep";
import SubstituteStep from "./SubstituteStep";

type StepsProps = {
  steps: SolutionTS;
};

type Step = {
  type: "accumulate" | "remove" | "substitute";
  step: ResultAccumulateStepTS | ResultRemoveStepTS | ResultSubstituteStepTS;
};

const Solution = ({ steps }: StepsProps) => {
  const all_steps = (steps: SolutionTS) => {
    let num_steps =
      steps.result_accumulate_steps.length +
      steps.result_remove_steps.length +
      steps.result_substitute_steps.length;
    let all_steps: React.ReactNode[] = [];
    for (let i = 0; i < num_steps; i++) {
      let step_accumulate = steps.result_accumulate_steps.find(
        (step) => step.id == i,
      );
      if (step_accumulate) {
        all_steps.push(<AccumulateStep step={step_accumulate} />);
      }
      let step_remove = steps.result_remove_steps.find((step) => step.id == i);
      if (step_remove) {
        all_steps.push(<RemoveStep step={step_remove} />);
      }
      let step_substitute = steps.result_substitute_steps.find(
        (step) => step.id == i,
      );
      if (step_substitute) {
        all_steps.push(<SubstituteStep step={step_substitute} />);
      }
    }
    return all_steps;
  };

  return (
    <Card title="Solution - Step by Step">
      <Box ps={"xl"} w={"100%"}>
        {all_steps(steps).map((step, index) => (
          <>
            <Box w={"100%"} pb={"md"}>
              {step}
            </Box>
            {index == all_steps(steps).length - 1 ? null : (
              <Divider key={index} py={"md"} />
            )}
          </>
        ))}
      </Box>
      {steps.result_error ? (
        <MathJax style={{ color: "red" }}>{steps.result_error}</MathJax>
      ) : null}
    </Card>
  );
};

export default Solution;
