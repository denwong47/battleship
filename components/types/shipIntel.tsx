import { Uuid } from './id'

export class ShipIntel {
    damages: number
    remaining: number
    shipType: string
    status: "Undiscovered" | "Discovered" | "Sunk"
    uuid: Uuid

    constructor(damages: number, remaining: number, shipType: string, status: "Undiscovered" | "Discovered" | "Sunk", uuid: Uuid) {
        this.damages = damages
        this.remaining = remaining
        this.shipType = shipType
        this.status = status
        this.uuid = uuid
    }

    static fromJson(json: any): ShipIntel | null{
        let uuid = Uuid.fromString(json.uuid)

        if (uuid !== null) {
            return new ShipIntel(json.damages, json.remaining, json.ship_type, json.status, uuid)
        } else {
            return null
        }
    }
}
