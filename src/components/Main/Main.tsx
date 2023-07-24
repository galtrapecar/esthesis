import React, { useEffect, useState } from "react";
import { Header } from "./Header/Header";
import { Phenotypes } from "./Phenotypes/Phenotypes";
import { invoke } from "@tauri-apps/api";

export function Main({ images }: { images: string[][], }) {
    return (
        <div className="Main">
            <Header />
            {/* Generate */}
            {/* if generating LoadingBar */}
            {/* if generated  Results */}
            <Phenotypes images={images}  />
        </div>
    )
}