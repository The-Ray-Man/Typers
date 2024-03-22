import { useEffect, useState } from 'react';
import './App.css';
import { TreeTS, parse_input } from 'FMFP';

function App() {

  const [input, setInput] = useState('');

  const [tree, setTree] = useState<TreeTS | null>(null);
  const [error, setError] = useState<string | null>(null);

  try {
    const tree = parse_input(input);
    setTree(tree.tree);
    setError(null);
  } catch (e ) {
    setError("invalid string");
  }

  return (
    <div className="App">
      <input type='text' value={input} onChange={(e) => setInput(e.target.value)} />
      {tree && (tree.expr)}
    </div>
  );
}

export default App;
