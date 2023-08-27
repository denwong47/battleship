import { BoardStatus } from "./types/boardStatus"
import InputButton from "./inputButton"

export interface BoardControlProps {
    boardState: BoardStatus | null,
    syncButtonAction: () => Promise<void>,
    backButtonAction: () => Promise<void>,
}

export default function BoardControl(props: BoardControlProps) {
    const genericClassNames = "flex-grow flex items-center justify-center rounded-lg bg-sky-500/[.06]"
    const buttonClassNames = "w-[var(--board-control-button-size)] aspect-square grow-0 hover:bg-sky-500/75 transition-all duration-300"

    return (
        <main className="flex w-full gap-[var(--position-cell-spacing)]">
            <div className={`flex-col ${buttonClassNames} ${genericClassNames}`}>
                <InputButton clickAction={props.backButtonAction} caption={'\u{2196}'} classNames="h-full w-full text-button" />
            </div>
            <div className={`flex-col grow-1 font-mono ${genericClassNames}`}>{props.boardState ? props.boardState.strikes : '\u{23F3}'}</div>
            <div className={`flex-col ${buttonClassNames} ${genericClassNames}`}>
                <InputButton clickAction={props.syncButtonAction} caption={'\u{21A9}'} classNames="h-full w-full text-button" />
            </div>
        </main>
    )
}
