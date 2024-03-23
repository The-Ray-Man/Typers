import { useEffect, useState } from 'react';
import { TreeTS, parse_input } from 'FMFP';
import { Center, TextInput, Stack, Title, Text } from '@mantine/core';
import Error from './components/Error';
import Constraints from './components/Constraints';
import Tree from './components/Tree';
import Variables from './components/Variables';
import Solution from './components/Solution';
import Steps from './components/steps/Steps';

function App() {

  const [input, setInput] = useState('');

  const parsed = parse_input(input);

  return (
    <div className="App">
      <Center>
        <Stack pt={"xl"}>
          <Center>

          <Title order={1}>
            FMFP Made easy
          </Title>
          </Center>


      <TextInput value={input} onChange={(e) => setInput(e.currentTarget.value)} />

      {parsed.parse_error && (<Error error={parsed.parse_error} />)}
      {parsed.build_tree_error && (<Error error={parsed.build_tree_error} />)}
      {parsed.tree && (<Tree mathjax={parsed.tree}/>)}
      {parsed.constraints_error && (<Error error={parsed.constraints_error} />)}
      {parsed.constraints && (<Constraints constraints={parsed.constraints} />)}
      {parsed.solution_error && (<Error error={parsed.solution_error} />)}
      {parsed.solution && parsed.solution.variables &&(<Variables variables={parsed.solution.variables}/>)}
      {parsed.solution && parsed.solution.result &&(<Solution solution={parsed.solution.result}/>)}
      {parsed.solution && parsed.solution &&(<Steps steps={parsed.solution}/>)}
      

        </Stack>
      </Center>
    </div>
  );
}

export default App;
