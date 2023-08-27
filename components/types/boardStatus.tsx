import React from 'react'
import { Uuid } from "./id";
import { ShipIntel } from "./shipIntel";

export interface UpdateOptions {
    active?: boolean | null;
    elapsed?: number | null;
    shipIntel?: Array<ShipIntel> | null;
    size?: [number, number] | null;
    strikes?: number | null;
    uuid?: Uuid | null;
}

export class BoardStatus {
    active: boolean
    elapsed: number
    shipIntel: Map<string, ShipIntel>
    size: [number, number]
    strikes: number
    uuid: Uuid
    setter: React.Dispatch<React.SetStateAction<BoardStatus | null>>

    constructor(
        active: boolean,
        elapsed: number,
        shipIntel: Array<ShipIntel>,
        size: [number, number],
        strikes: number,
        uuid: Uuid,
        setter: React.Dispatch<React.SetStateAction<BoardStatus | null>>,
    ) {
        this.active = active
        this.elapsed = elapsed
        this.shipIntel = new Map(shipIntel.map((ship: ShipIntel) => [ship.uuid.toString(), ship]))
        this.size = size
        this.strikes = strikes
        this.uuid = uuid
        this.setter = setter
    }

    static fromJson(json: any, setter: React.Dispatch<React.SetStateAction<BoardStatus | null>>): BoardStatus | null {
        let uuid = Uuid.fromString(json.uuid)

        if (uuid !== null) {
            let shipIntel = json.ship_intel.map((ship: any) => ship !== null ? ShipIntel.fromJson(ship) : null)

            if (shipIntel.every((intel: ShipIntel | null) => intel !== null)) {
                return new BoardStatus(
                    json.active,
                    json.elapsed,
                    shipIntel as Array<ShipIntel>,
                    json.size,
                    json.strikes,
                    uuid,
                    setter,
                )
            } else {
                return null
            }
        } else {
            return null
        }
    }

    update = (options: UpdateOptions): BoardStatus => {
        const newInstance = new BoardStatus(
            options.active ?? this.active,
            options.elapsed ?? this.elapsed,
            Array.isArray(options.shipIntel) ? options.shipIntel : Array.from(this.shipIntel.values()),
            options.size ?? this.size,
            options.strikes ?? this.strikes,
            options.uuid ?? this.uuid,
            this.setter, // Cannot change setter.
        )

        this.setter(newInstance)

        return newInstance
    }

    freeze = (): BoardStatus => {
        return this.update({ active: false })
    }

    updatedShipIntels = (ship: ShipIntel): Map<string, ShipIntel> => {
        const key = ship.uuid.toString()

        if (this.shipIntel.has(key)) {
            this.shipIntel.set(key, ship)
        } else {
            console.error(
                `Cannot update ship '${ship.uuid.toString()}': not found on board '${this.uuid.toString()}'.}`
            )
        }
        return this.shipIntel
    }
}
