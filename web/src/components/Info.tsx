import { Title, Text, Textarea, Flex, Divider } from "@mantine/core";
import Card from "./Card";
import { MathJax } from "better-react-mathjax";
import MiniHaskell from "./MiniHaskell";

const Info = () => {
  return (
    <Card title="">
      <Title order={4}>Info</Title>
      <Textarea readOnly w={"100%"} autosize variant="unstyled">
        This tool is based on the ETHZ course Formal Methods and Functional
        Programming. Its goal is to help you understand the type inference
        process of the Mini-Haskell language.
      </Textarea>

      <Title order={4}>Mini-Haskell</Title>
      <Text>Mini Haskell consists of the following rules:</Text>
      <MiniHaskell />

      <Title order={4}>How does it work?</Title>
      <Textarea readOnly w={"100%"} autosize variant="unstyled">
        The tool takes a Mini-Haskell expression as input and parses it into a
        Abstract Syntax Tree (AST). For this we wrote our own Grammar. Its not
        clear that this grammar is exactly the grammar which the creators of
        Mini-Haskell had in mind.
      </Textarea>
      <Textarea readOnly w={"100%"} autosize variant="unstyled">
        With the AST we can now create a type inference tree. This tree is
        created by recursively applying the type inference rules to the AST.
      </Textarea>
      <Textarea readOnly w={"100%"} autosize variant="unstyled">
        Using this tree we can generate the constraints for the type of the
        expression. The Algorithm has about three actions:
      </Textarea>
      <Textarea readOnly w={"100%"} autosize variant="unstyled">
        1. Accumulate: In this step we match two rules and infer more
        constraints
      </Textarea>
      <Textarea readOnly w={"100%"} autosize variant="unstyled">
        2. Remove: In this step we choose a simple rule (e.g. t=Int) and replace
        every occurrence of t with the rhs of the rule
      </Textarea>
      <Textarea readOnly w={"100%"} autosize variant="unstyled">
        3. Substitute: In this step we infer the type of the expression by
        substituting types with more concrete types.
      </Textarea>
      <Title order={4}>Contribution</Title>
      <Textarea readOnly w={"100%"} autosize variant="unstyled">
        This tool was developed by students of the ETHZ course Formal Methods
        and Functional Programming. Feel free to contribute to the project by
        visiting the GitHub repository. If you find mistakes please open an
        issue on Github.
      </Textarea>
    </Card>
  );
};

export default Info;
