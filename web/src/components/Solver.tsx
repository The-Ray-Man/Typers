import { useEffect, useState } from "react";
import { TreeTS, solve } from "FMFP";
import {
  Center,
  TextInput,
  Stack,
  Title,
  Text,
  Box,
  Flex,
  Switch,
  Button,
} from "@mantine/core";
import Card from "./Card";
import Error from "./Error";
import Constraints from "./Constraints";
import Tree from "./Tree";
import Variables from "./Variables";
import Solution from "./Solution";
import Steps from "./steps/Steps";
import Info from "./Info";
import MiniHaskell from "./MiniHaskell";

const Solver = () => {
  const [input, setInput] = useState("");

  const parsed = solve(input);

  const [showHaskell, setShowHaskell] = useState(false);

  return (
    <>
      <Center>
        <Stack pt={"xl"} pb={"xl"} justify="space-between">
          <Box>
            <TextInput
              placeholder="(\x -> (\y -> (x y)))"
              value={input}
              onChange={(e) => setInput(e.currentTarget.value)}
              miw={600}
              pt={"md"}
              pb={0}
            />
            <Text pt={0} c={"dimmed"}>
              Every expression except primitive types
              have to be in brackets
            </Text>
            <Flex gap={"md"} justify={"space-around"} align={"center"}>
              <Button
                variant="transparent"
                onClick={() => setInput("(\\x -> (\\y -> (x y)))")}
              >
                Simple Example
              </Button>
              <Button
                variant="transparent"
                onClick={() =>
                  setInput(
                    "(\\x -> (\\y -> (if (iszero y) then (x + y) else (x * y))))",
                  )
                }
              >
                Medium Example
              </Button>
              <Button
                variant="transparent"
                onClick={() =>
                  setInput(
                    "(\\x -> (if ((snd x) 1) then (\\y -> ((fst x) y)) else (\\z -> (iszero ((z +1) * 3)))))",
                  )
                }
              >
                Complex Example
              </Button>

              {input.length > 0 && (
                <Switch
                  checked={showHaskell}
                  onChange={() => setShowHaskell(!showHaskell)}
                  label="Show Rules"
                  labelPosition="left"
                />
              )}
            </Flex>
          </Box>

          {input.length === 0 && <Info />}

          {showHaskell && input.length !== 0 && (
            <Card title="Mini-Haskell">
              <MiniHaskell />
            </Card>
          )}

          {input.length > 0 && (
            <>
              {parsed.parse_error && <Error error={parsed.parse_error} />}
              {parsed.build_tree_error && (
                <Error error={parsed.build_tree_error} />
              )}
              {parsed.tree && <Tree mathjax={parsed.tree} />}
              {parsed.constraints_error && (
                <Error error={parsed.constraints_error} />
              )}
              {parsed.constraints && parsed.constraints_without_trivial && (
                <Constraints
                  constraints={parsed.constraints}
                  constraints_without_trivial={
                    parsed.constraints_without_trivial
                  }
                />
              )}
              {parsed.solution && parsed.solution.variables && (
                <Variables variables={parsed.solution.variables} />
              )}
              {parsed.solution && parsed.solution.result && (
                <Solution solution={parsed.solution.result} />
              )}
              {parsed.solution && parsed.solution && (
                <Steps steps={parsed.solution} />
              )}
            </>
          )}
        </Stack>
      </Center>
    </>
  );
};

export default Solver;
