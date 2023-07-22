import React, { useEffect, useRef, useState } from "react";
import { fitness, init, new_population } from "../../genetic/genetic";
import './Population.css'
import { IGenotype } from "../../genetic/helpers";

export function Population({setLoading}: {setLoading: React.Dispatch<React.SetStateAction<boolean>>}) {
    const [phenotypes, setPhenotypes] = useState(init())
    const firedRef = useRef(false);

    useEffect(() => {
        if (firedRef.current) return
        firedRef.current = true

        // window.addEventListener('click', evolve)
        evolve().then(_ => setLoading(false))
    }, [])

    function evolve() {
        return new Promise<boolean>(async (resolve, reject) => {
            let genotypes = [...phenotypes]

            let i = 0
            while (i < 10000) {
                await new Promise(r => setTimeout(r, 20));
        
                let population = [...new_population(genotypes)]
                setPhenotypes(population)

                if (population.length == 16) break
                
                genotypes = population

                i++
            }

            resolve(true)
        })
    }

    return (
        <div className="Population">
            <div className="inner">
                {phenotypes.map(X => {
                    return (
                        <div className="bar" key={`${Math.floor(Math.random() * 100000000)}`} style={{backgroundColor: X.color, height: X.height}}>
                            {X.fitness}    
                        </div>
                    )
                })}
            </div>
        </div>
    )
}