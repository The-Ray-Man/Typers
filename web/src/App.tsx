import { useEffect } from 'react';
import './App.css';
import __wbg_init from './wasm/wasm';

function App() {
  
  useEffect(() => {
    __wbg_init().then((wasm) => {
      console.log(wasm.hello_world());
    });
  }, []);

  return (
    <div className="App">

    </div>
  );
}

export default App;
