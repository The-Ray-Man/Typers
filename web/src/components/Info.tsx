import { Title, Text, Textarea, Flex, Divider } from "@mantine/core";
import Card from "./Card";
import { MathJax } from "better-react-mathjax";
import MiniHaskell from "./MiniHaskell";


const Info = () => {
    return (
        <Card title="">
            <Title order={4}>Info</Title>
            <Textarea readOnly w={"100%"} autosize variant="unstyled">
                This tool is based on the ETHZ course Formal Methods and Functional Programming. 
                Its goal is to help you understand the type inference process of the Mini-Haskell language.
            </Textarea>

            <Title order={4}>Mini-Haskell</Title>
            <Text>Mini Haskell consists of the following rules:</Text>
            <MiniHaskell/>
            <Divider />

            <Title order={4}>Contribution</Title>
            <Textarea readOnly w={"100%"} autosize variant="unstyled">
                This tool was developed by students of the ETHZ course Formal Methods and Functional Programming.
                Feel free to contribute to the project by visiting the GitHub repository. If you find mistakes please open an issue on Github.

                </Textarea>

        </Card>

    );
    }

export default Info;