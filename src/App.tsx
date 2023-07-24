import React, { useEffect, useState } from 'react'
import "./App.css";
import { Population } from "./components/Population/Population";
import { Main } from './components/Main/Main';
import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';

function App() {
  const [loading, setLoading] = useState(true)
  const [listenPaused, pauseListen] = useState(false)
  const [images, setImages] = useState<string[][]>([[]])

  const getPhenotypes = async () => {
    const images = await invoke("get_phenotypes")
    console.log(images);
    
    setImages(images as any)
  }

  useEffect(() => {
    const unlisten = listen("loading", async (e) => {
      if (listenPaused) return
      pauseListen(true)
      await getPhenotypes()
      setLoading(false)
    });
    return () => {
      unlisten.then(f => f())
    };
  }, []);

  return (
    <div className="container">
      {
        loading || images.length === 0 ? 
          <Population />
        :
          <Main images={images}  />
      }
    </div>
  );
}

export default App