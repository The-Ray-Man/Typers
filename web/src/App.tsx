import { useEffect, useState } from 'react';
import { TreeTS, parse_input } from 'FMFP';
import { Center, TextInput, Stack, Title } from '@mantine/core';
import Error from './components/Error';
import Constraints from './components/Constraints';
import Tree from './components/Tree';

function App() {

  const [input, setInput] = useState('');

  const tree = parse_input(input);

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

      {tree.parse_error && (<Error error={tree.parse_error} />)}
      {tree.build_tree_error && (<Error error={tree.build_tree_error} />)}
      {tree.tree && (<Tree mathjax={tree.tree}/>)}
      {tree.constraints_error && (<Error error={tree.constraints_error} />)}
      {tree.constraints && (<Constraints constraints={tree.constraints} />)}

        </Stack>
      </Center>
    </div>
  );
}

export default App;
