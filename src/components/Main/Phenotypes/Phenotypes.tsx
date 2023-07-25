import React, { useContext, useEffect, useState } from "react"
import "./Phenotypes.css"
import { context } from "../../../App"

function Phenotype({ image, id, selected, setSelectedPhenotype }: { image: string, id: string, selected: boolean, setSelectedPhenotype: (string: string) => void }) {
    return (
        <div className={`Phenotypes__image ${selected ? "selected" : ""}`} onClick={() => {
            setSelectedPhenotype(id)
        }}>
            <img src={image} />
        </div>
    )
}

export function Phenotypes({ images }: { images: string[][] }) {
    const [globalState, setGlobalState] = useContext(context)
    const [selectedPhenotypes, setSelectedPhenotypes] = useState<string[]>([])

    const setSelectedPhenotypesCallback = (string: string) => {
        if (selectedPhenotypes.length == 2) {
            let old = selectedPhenotypes.pop();
            selectedPhenotypes.pop();
            setSelectedPhenotypes([old as string, string])
            setGlobalState(() => {
                let new_state = { ...globalState }
                new_state.selected = [old as string, string]
                return new_state
            })
        } else {
            setSelectedPhenotypes([...selectedPhenotypes, string])
            setGlobalState(() => {
                let new_state = { ...globalState }
                new_state.selected = [...selectedPhenotypes, string]
                return new_state
            })
        }
    }

    return (
        <div className="Phenotypes">
            {images.map(image => (
                <Phenotype image={image[0]} id={image[1]} selected={selectedPhenotypes.includes(image[1])} key={image[1]} setSelectedPhenotype={setSelectedPhenotypesCallback} />
            ))}
        </div>
    )
}
