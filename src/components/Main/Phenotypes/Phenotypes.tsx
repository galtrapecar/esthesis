import React, { useEffect, useState } from "react"
import "./Phenotypes.css"



export function Phenotypes({ images }: { images: string[][] }) {
    const [selectedPhenotype, setSelectedPhenotype] = useState("")
    
    function Phenotype({ image, id }: { image: string, id: string }) {
        const [selected, setSelected] = useState(false)
        return (
            <div className={`Phenotypes__image ${selected ? "selected" : ""}`} onClick={() => {
                setSelected(!selected)
                setSelectedPhenotype(id)
            }}>
                <img src={image} />
            </div>
        )
    }

    return (
        <div className="Phenotypes">
            {images.map(image => (
                <Phenotype image={image[0]} id={image[1]} />
            ))}
        </div>
    )
}
