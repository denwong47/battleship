export interface ToolTipProps {
    text: string,
    classNames: string,
}
export default function ToolTip(props: ToolTipProps) {
    return (
        <div className={"tooltip "+props.classNames}>
            {props.text}
        </div>
    )
}
