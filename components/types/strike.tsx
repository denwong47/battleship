import { ShipIntel } from "./shipIntel"
import { Uuid } from "./id"

export class Strike {
    coordinates: {
        x: number,
        y: number,
    }
    hit: boolean
    intel: ShipIntel | null
    ships_remaining: number
    uuid: Uuid

    constructor(coordinates: { x: number, y: number }, hit: boolean, intel: ShipIntel | null, ships_remaining: number, uuid: Uuid) {
        this.coordinates = coordinates
        this.hit = hit
        this.intel = intel
        this.ships_remaining = ships_remaining
        this.uuid = uuid
    }

    static fromJson(json: any): Strike | null {
        if (json !== null) {
            let uuid = Uuid.fromString(json.uuid)

            if (uuid !== null) {
                let intel = json.intel !== null ? ShipIntel.fromJson(json.intel) : null

                return new Strike(json.coordinates, json.hit, intel, json.ships_remaining, uuid)
            }
        }

        return null
    }
}
