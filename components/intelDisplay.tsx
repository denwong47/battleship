import { BoardStatus } from "@/components/types/boardStatus";
import { ShipIntel } from "@/components/types/shipIntel";
import ShipDisplay from "./shipDisplay";

import BoardControl from "./boardControl";

export interface IntelDisplayProps {
    boardState: BoardStatus | null,
    backToLobby: () => Promise<void>,
    syncBoardState: () => Promise<void>,
}

export default function IntelDisplay(props: IntelDisplayProps) {
    const ships = props.boardState ? Array.from(props.boardState?.shipIntel.values()) : []

    return (
        <main className="table-cell border-spacing-y-0.5 intel-display md:min-w-[240px] min-w-full">
            <BoardControl
                boardState={props.boardState}
                backButtonAction={props.backToLobby}
                syncButtonAction={props.syncBoardState}
            />
            <div className="table overflow-y-auto border-spacing-y-[8px]">
                {
                    Array.from(
                        ships.map((ship: ShipIntel) => {
                            return <ShipDisplay key={ship.uuid.toString()} shipIntel={ship} />
                        })
                    )
                }
            </div>
        </main>
    )
}
