export type PositionState = {
    char: string,
    className: string,
    onClick: (x: number, y: number) => void,
}

export const UnknownState: PositionState = {
    char: '',
    className: 'bg-sky-500/25 hover:bg-yellow-500/75 duration-1000 hover:ease-out hover:drop-shadow-[0_0_1.4rem_#ff0000ff] cursor-pointer',
    onClick: (x: number, y: number) => { console.log(`Default action triggered: Clicked ${x}, ${y}`) },
}

export const MissedState: PositionState = {
    char: '\u{1F300}',
    className: 'bg-sky-800/25 cursor-not-allowed',
    onClick: (x: number, y: number) => { console.log(`Default action triggered: Clicked ${x}, ${y}`) },
}

export const HitState: PositionState = {
    char: '\u{1F4A5}',
    className: 'bg-red-500/75 drop-shadow-[0_0_1.4rem_#ff0000ff] cursor-not-allowed',
    onClick: (x: number, y: number) => { console.log(`Default action triggered: Clicked ${x}, ${y}`) },
}
