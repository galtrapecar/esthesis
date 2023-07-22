import React, { useState } from 'react'
import "./App.css";
import { Population } from "./components/Population/Population";
import { Main } from './components/Main/Main';

function App() {
  const [loading, setLoading] = useState(false)

  return (
    <div className="container">
        {<Population setLoading={setLoading} /> }
        {/* <Main /> */}
    </div>
  );
}

export default App