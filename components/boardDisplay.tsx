import { BoardStatus, UpdateOptions } from "@/components/types/boardStatus"

import { Strike } from "./types/strike";
import { UnknownState, MissedState, HitState, PositionState } from "./types/positionState";
import { apiURL } from "./config";
import { notifier } from "@/app/notifier";

export type BoardDisplayProps = {
    boardState: BoardStatus | null,
    strikes: Array<Strike>,
    addStrike: (strike: Strike) => void,
}

export default function BoardDisplay(props: BoardDisplayProps) {
    const { boardState, strikes, addStrike } = props

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

                            let options: UpdateOptions = {
                                active: strike.ships_remaining > 0,
                                strikes: boardState.strikes + 1, // Update strikes regardless of hit
                            }

                            if (strike.intel !== null) {
                                options.shipIntel = Array.from(boardState.updatedShipIntels(strike.intel).values())

                                const shipType = strike.intel.shipType
                                const indefiniteArticle = ['a', 'e', 'i', 'o', 'u'].includes(shipType[0].toLowerCase()) ? 'an' : 'a'
                                notifier.info(
                                    `You ${strike.intel.remaining >0 ? 'hit' : 'sank'} ${indefiniteArticle} ${shipType}!`,
                                    {
                                        labels: {
                                            info: "\u{2757} Ship " + (strike.intel.remaining > 0 ? 'hit' : 'sank'),
                                        }
                                    }
                                )
                            }

                            if (strike.ships_remaining === 0) {
                                notifier.success(
                                    `You sank all the ships in ${options.strikes} strikes! Congratulations!`,
                                    {
                                        labels: {
                                            success: "\u{2705} Game won",
                                        }
                                    }
                                )
                            }

                            // Finally apply the updates.
                            boardState.update(options)
                        }
                    } else {
                        console.error(`Failed to strike at ${x}, ${y}: ${response.status} ${await response.json()}`)
                    }
                }
            ) : (
                async (x: number, y: number) => {
                    if (boardState.active) {
                        notifier.alert(
                            "You have already struck there.",
                            {
                                labels: {
                                    alert: "Already struck",
                                }
                            }
                        )
                    } else {
                        notifier.info(
                            "Game has already won.",
                            {
                                labels: {
                                    info: "No more ships",
                                }
                            }
                        )
                    }
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

        console.log(boardState)

        return (
            <main className="grow flex justify-center">
            <div className="flex-table self-center justify-center w-fit" style={{ gridTemplateColumns: `repeat(${gridWidth}, minmax(0, var(--position-cell-width)))`, gridTemplateRows: `repeat(${gridHeight}, minmax(0, 1fr))` }}>
            {
                Array.from(Array.from(Array(gridWidth * gridHeight)), (_, index) => {
                    const x = index % gridWidth
                    const y = Math.floor(index / gridWidth)
                    const position = gridMap[x][y]
                    return (
                    <div key={index} className="flex-table-cell w-fit">
                        <div className={'table rounded-lg aspect-square w-[var(--position-cell-width)] text-center transition-all duration-1000 ' + position.className} onClick={(event) => position.onClick(x, y)}>
                        <div className='table-cell align-middle text-icon overflow-hidden'>{position.char}</div>
                        </div>
                    </div>
                    )
                })
            }
            </div>
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
