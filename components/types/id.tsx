import { parse as uuidParse, stringify as uuidStringify, validate as uuidValidate, v4 as uuidV4 } from 'uuid'

export class Uuid {
    readonly bytes: Uint8Array[16]

    constructor(bytes: Uint8Array[16]) {
        this.bytes = bytes
    }

    static isValid(str: string): boolean {
        return uuidValidate(str)
    }

    static fromString(str: string): Uuid | null {
        if (Uuid.isValid(str)) {
            return new Uuid(uuidParse(str))
        } else {
            console.error(`Invalid UUID: ${str}`)
            return null
        }
    }

    static new(): Uuid {
        return new Uuid(uuidV4())
    }

    toString(): string {
        return uuidStringify(this.bytes)
    }
}
