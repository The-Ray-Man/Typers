import { Center, Flex, Grid, SimpleGrid, Stack, Text, Title } from "@mantine/core";
import { IconArrowRight } from "@tabler/icons-react";
import { ResultSubstituteStepTS } from "FMFP";
import { MathJax } from "better-react-mathjax";


type SubstituteStepProps = {
    step : ResultSubstituteStepTS;
}


const SubstituteStep = ({ step } : SubstituteStepProps) => {
    

    return (
        <Stack w={"100%"}>
        <Center>

        <Title order={3}>Substitute Step</Title>
        </Center>
        <Flex justify={"space-around"}  w={"100%"}>
            <Stack>
                <Text>Rules Available</Text>
                {step.rules_available.map((rule, index) => {
if (rule == step.rule_used) {
                        return (
                            <MathJax style={{color: "green"}}>{"\\("+rule+"\\)"}</MathJax>
                        )
                    }
                    return (
                            <MathJax>{"\\("+rule+"\\)"}</MathJax>
                    )
                })}
            </Stack>


            <Stack>
                <Text>Goal Rule</Text>
          
            <MathJax>{"\\("+step.rule_goal_before + "\\)"}</MathJax>

                <Text>Used Rule to Replace</Text>
                <MathJax style={{color:"green"}}>{"\\("+step.rule_used+"\\)"}</MathJax>
            </Stack>
            <Stack>
                
                <Text>New Goal Rule</Text>
            <MathJax>{"\\("+step.rule_goal_after+"\\)"}</MathJax>
                
            </Stack>

        </Flex>
        </Stack>
    )

}

export default SubstituteStep;
