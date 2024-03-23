import { useEffect, useState } from 'react';
import { TreeTS, parse_input } from 'FMFP';
import { Center, TextInput, Stack, Title, Text, Box, Flex, AppShell } from '@mantine/core';
import Error from './components/Error';
import Constraints from './components/Constraints';
import Tree from './components/Tree';
import Variables from './components/Variables';
import Solution from './components/Solution';
import Steps from './components/steps/Steps';
import Solver from './components/Solver';
import Footer from './components/Footer';

function App() {

  return (
    <div className="App">
      <AppShell pt={"xl"}>
      <Center>

<Title order={1}>
  FMFP Type Inference Made Easy
</Title>
</Center>
        <AppShell.Main>
      <Solver />
        </AppShell.Main>
        <div style={{height:"100"}}></div>
    <AppShell.Footer>
      <Footer />
    </AppShell.Footer>
      </AppShell>
    </div>
  );
}

export default App;
