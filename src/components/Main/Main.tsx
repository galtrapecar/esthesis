import React, { useEffect, useState } from "react";
import { Header } from "./Header/Header";
import { Phenotypes } from "./Phenotypes/Phenotypes";
import { invoke } from "@tauri-apps/api";

export function Main() {
    const [images, setImages] = useState<string[]>([])

    const getPhenotypes = async () => {
        const images = await invoke("get_phenotypes")
        console.log(images);
        
        setImages(images as any)
    }

    useEffect(() => {
        getPhenotypes()
    }, [])

    return (
        <div className="Main">
            <Header />
            {/* Generate */}
            {/* if generating LoadingBar */}
            {/* if generated  Results */}
            <Phenotypes images={images} />
        </div>
    )
}