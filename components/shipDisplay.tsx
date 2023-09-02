import React from "react";
import { ShipIntel } from "./types/shipIntel"

export interface ShipDisplayProps {
    shipIntel: ShipIntel;
}

const backgroundTypes = {
    "Undiscovered": "bg-sky-500/[.06]",
    "Discovered": "bg-yellow-500/[.24]",
    "Sunk": "bg-red-500/[.24]",
}

export default function ShipDisplay(props: ShipDisplayProps) {
    const damageIcons = '\u{274C}'.repeat(props.shipIntel.damages) + '\u{1F7E6}'.repeat(props.shipIntel.remaining)

    return (
        <div className="table-row">
            <div className={`table-cell rounded-lg p-4 transition duration-1000 ${backgroundTypes[props.shipIntel.status]} bg-no-repeat bg-image-ship bg-[90%_92%] bg-origin-padding`} style={{backgroundImage: `url('/images/ship-${props.shipIntel.shipType.toLowerCase()}.png')`}}>
            <p className="text-base">{props.shipIntel.shipType}</p>
            <p className="text-xxs font-mono text-sky-500/25">{props.shipIntel.uuid.toString()}</p>
            <p className="pt-6 text-sm tracking-widest">{damageIcons}</p>
            </div>
        </div>
    )
}
