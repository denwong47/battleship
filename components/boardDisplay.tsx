import { BoardStatus } from "@/components/types/boardStatus"

import { Strike } from "./types/strike";
import { UnknownState, MissedState, HitState, PositionState } from "./types/positionState";
import { apiURL } from "./config";

export type BoardDisplayProps = {
    boardState: BoardStatus | null,
    strikes: Array<Strike>,
    addStrike: (strike: Strike) => void,
    freezeBoard: () => void,
}

export default function BoardDisplay(props: BoardDisplayProps) {
    const { boardState, strikes, addStrike, freezeBoard } = props

    function createGridMap(boardState: BoardStatus, strikes: Array<Strike>): Array<Array<PositionState>> {
        const gridWidth = boardState?.size[0] ?? 10
        const gridHeight = boardState?.size[1] ?? 10

        function makeState(baseState: PositionState): PositionState {
            const callback = (boardState.active && baseState === UnknownState) ? (
                // Make a strike.
                async (x: number, y: number) => {
                    const response = await fetch(`${apiURL}/strike/${boardState.uuid}?x=${x}&y=${y}`)

                    if (response.ok) {
                        const strike = Strike.fromJson(await response.json())
                        if (strike !== null) {
                            addStrike(strike)
                            if (strike?.ships_remaining === 0) {
                                freezeBoard()
                            }
                        }
                    } else {
                        console.error(`Failed to strike at ${x}, ${y}: ${response.status} ${await response.json()}`)
                    }
                }
            ) : (
                async (x: number, y: number) => {
                    // Do nothing. The strike is already done.
                }
            )

            return {
                ...baseState,
                onClick: async (x: number, y:number) => {
                    try {
                        await callback(x, y)
                    } catch (error) {
                        console.error(error)
                    }
                }
            }
        }

        let gridMap = strikes.reduce((gridMap, strike) => {
            gridMap[strike.coordinates.x][strike.coordinates.y] = strike.hit ? makeState(HitState) : makeState(MissedState)
            return gridMap
        }, Array.from({ length: gridWidth }, () => Array(gridHeight).fill(makeState(UnknownState))))

        return gridMap
    }

    if (boardState !== null) {
        const gridWidth = boardState?.size[0] ?? 10
        const gridHeight = boardState?.size[1] ?? 10

        const gridMap = createGridMap(boardState, strikes)

        return (
            <main className="flex-table" style={{ gridTemplateColumns: `repeat(${gridWidth}, minmax(0, 4rem))`, gridTemplateRows: `repeat(${gridHeight}, minmax(0, 1fr))` }}>
            {
                Array.from(Array.from(Array(gridWidth * gridHeight)), (_, index) => {
                    const x = index % gridWidth
                    const y = Math.floor(index / gridWidth)
                    const position = gridMap[x][y]
                    return (
                    <div key={index} className="flex-table-cell w-fit">
                        <div className={'table rounded-lg aspect-square w-16 text-center ' + position.className} onClick={(event) => position.onClick(x, y)}>
                        <div className='table-cell align-middle text-4xl'>{position.char}</div>
                        </div>
                    </div>
                    )
                })
            }
            </main>
        )
    } else {
        return (
            <main className="flex-table">
                <p>Loading board state...</p>
            </main>
        )
    }
}
