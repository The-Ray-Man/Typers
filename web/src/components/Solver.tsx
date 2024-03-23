import { useEffect, useState } from 'react';
import { TreeTS, parse_input } from 'FMFP';
import { Center, TextInput, Stack, Title, Text, Box, Flex, Switch } from '@mantine/core';
import Card from './Card';
import Error from './Error';
import Constraints from './Constraints';
import Tree from './Tree';
import Variables from './Variables';
import Solution from './Solution';
import Steps from './steps/Steps';
import Info from './Info';
import MiniHaskell from './MiniHaskell';

const Solver = () =>  {

  const [input, setInput] = useState('');

  const parsed = parse_input(input);


  const [showHaskell, setShowHaskell] = useState(false);

  return (
    <>
      <Center>
        <Stack pt={"xl"} pb={"xl"} justify='space-between'>
    

      <TextInput placeholder='\x -> \y -> (x y)' value={input} onChange={(e) => setInput(e.currentTarget.value)} miw={600}  pt={"md"}/>
      <Flex justify={"end"} >

      <Switch checked={showHaskell} onChange={() => setShowHaskell(!showHaskell)} label="Show Rules" 
      labelPosition="left"/>
      </Flex>


      {input.length === 0 && (<Info/>)}

      {showHaskell && (<Card title='Mini-Haskell'><MiniHaskell/></Card>)}

      {input.length > 0 && (
          <>
          
          {parsed.parse_error && (<Error error={parsed.parse_error} />)}
      {parsed.build_tree_error && (<Error error={parsed.build_tree_error} />)}
      {parsed.tree && (<Tree mathjax={parsed.tree}/>)}
      {parsed.constraints_error && (<Error error={parsed.constraints_error} />)}
      {parsed.constraints && (<Constraints constraints={parsed.constraints} />)}
      {parsed.solution_error && (<Error error={parsed.solution_error} />)}
      {parsed.solution && parsed.solution.variables &&(<Variables variables={parsed.solution.variables}/>)}
      {parsed.solution && parsed.solution.result &&(<Solution solution={parsed.solution.result}/>)}
      {parsed.solution && parsed.solution &&(<Steps steps={parsed.solution}/>)}
      
        </>
          )}

        </Stack>
      </Center>
          </>
  );
}

export default Solver;
