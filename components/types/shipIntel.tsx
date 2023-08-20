import { Uuid } from './id'

export class ShipIntel {
    damages: number
    ship_type: string
    status: "Undiscovered" | "Discovered" | "Sunk"
    uuid: Uuid

    constructor(damages: number, ship_type: string, status: "Undiscovered" | "Discovered" | "Sunk", uuid: Uuid) {
        this.damages = damages
        this.ship_type = ship_type
        this.status = status
        this.uuid = uuid
    }

    static fromJson(json: any): ShipIntel | null{
        let uuid = Uuid.fromString(json.uuid)

        if (uuid !== null) {
            return new ShipIntel(json.damages, json.ship_type, json.status, uuid)
        } else {
            return null
        }
    }
}
