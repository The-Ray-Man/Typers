import { Center, Code, Flex, Text, Textarea } from "@mantine/core";
import { MathJax } from "better-react-mathjax";

import Card from "./Card";

type VariablesProps = {
  variables: string[];
};

const Variables = ({variables} : VariablesProps) => {
    return (
        <Card title="Variables">
            <Center>

            {variables.length === 0 && <Text c="red.2">No variables</Text>}
            </Center>
            <Flex align={"center"} gap={"md"}>

            {variables.map((variable, index) => (
                <MathJax key={index}>{"\\(" +  variable + "\\)"}</MathJax>
                ))}
                </Flex>

        </Card>
    );
    }

export default Variables;