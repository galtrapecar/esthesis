import React, { createContext, useContext, useEffect, useState } from 'react'
import "./App.css";
import { Population } from "./components/Population/Population";
import { Main } from './components/Main/Main';
import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';

type GlobalState = {
  selected: string[]
}

const defaultState: GlobalState = {
  selected: []
}

export type IGlobalStateContext = [GlobalState, React.Dispatch<React.SetStateAction<GlobalState>>];
export const context = React.createContext<IGlobalStateContext>([defaultState, () => null]);

function App() {

  const [globalState, setGlobalState] = useState<GlobalState>(defaultState)

  const [loading, setLoading] = useState(true)
  const [listenPaused, pauseListen] = useState(false)
  const [images, setImages] = useState<string[][]>([[]])

  const getPhenotypes = async () => {
    const images = await invoke("get_phenotypes")
    console.log(images);
    
    setImages(images as any)
  }

  const evolve = async () => {

  }

  useEffect(() => {
    const loading_unlisten = listen("loading", async (e) => {
      if (listenPaused) return
      pauseListen(true)

      await getPhenotypes()

      setLoading(false)
      pauseListen(false)
    });
    const evolving_unlisten = listen("evolving", async (e) => {
      setLoading(true)
      setImages([])
    });
    return () => {
      loading_unlisten.then(f => f())
    };
  }, []);

  return (
    <context.Provider value={[globalState, setGlobalState]}>
      <div className="container">
        {
          loading || images.length === 0 ? 
            <Population />
          :
            <Main images={images}  />
        }
      </div>
    </context.Provider>
  );
}

export default App