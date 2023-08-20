import { Uuid } from "./id";
import { ShipIntel } from "./shipIntel";

export class BoardStatus {
    active: boolean
    elapsed: number
    ship_intel: Map<string, ShipIntel>
    size: [number, number]
    strikes: number
    uuid: Uuid

    constructor(active: boolean, elapsed: number, ship_intel: Array<ShipIntel>, size: [number, number], strikes: number, uuid: Uuid) {
        this.active = active
        this.elapsed = elapsed
        this.ship_intel = new Map(ship_intel.map((ship: ShipIntel) => [ship.uuid.toString(), ship]))
        this.size = size
        this.strikes = strikes
        this.uuid = uuid
    }

    static fromJson(json: any): BoardStatus | null {
        let uuid = Uuid.fromString(json.uuid)

        if (uuid !== null) {
            let ship_intel = json.ship_intel.map((ship: any) => ship !== null ? ShipIntel.fromJson(ship) : null)

            if (ship_intel.every((intel: ShipIntel | null) => intel !== null)) {
                return new BoardStatus(json.active, json.elapsed, ship_intel as Array<ShipIntel>, json.size, json.strikes, uuid)
            } else {
                return null
            }
        } else {
            return null
        }
    }

    updateShipIntel = (ship: ShipIntel): BoardStatus => {
        this.ship_intel.set(ship.uuid.toString(), ship)

        return this
    }
}
