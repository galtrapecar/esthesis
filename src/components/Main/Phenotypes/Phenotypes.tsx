import React from "react"
import "./Phenotypes.css"

export function Phenotypes({ images }: { images: string[] }) {
    return (
        <div className="Phenotypes">
            {images.map(image => (
                <div className="Phenotypes__image">
                    <img src={image} />
                </div>
            ))}
        </div>
    )
}
