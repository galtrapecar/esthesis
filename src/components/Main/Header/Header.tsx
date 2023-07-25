import React from "react";
import './Header.css'
import { Logo } from "./Logo/Logo";
import { Button } from "./Button/Button";

export function Header() {
    return (
        <div className="Header">
            <Logo />
            <Button />
        </div>
    )
}