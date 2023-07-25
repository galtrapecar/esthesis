import React, { useContext, useEffect, useState } from "react"
import "./Button.css"
import { context } from "../../../../App";
import { invoke } from "@tauri-apps/api";

export function Button() {
    const [globalState, setGlobalState] = useContext(context)
    const [disabled, setDisabled] = useState(true)
    const [evolving, setEvolving] = useState(false)

    useEffect(() => {
        setDisabled(!(globalState.selected.length >= 2))
    }, [globalState])

    return (
        <div className={`Button ${disabled || evolving ? "disabled" : ""}`} onClick={async () => {
            setEvolving(true)
            await invoke("evolve_population", { selection: globalState.selected })
            setEvolving(false)
            setGlobalState(() => {
                let new_state = { ...globalState }
                new_state.selected = []
                return new_state
            })
        }}>
            Evolve
        </div>
    )
}
