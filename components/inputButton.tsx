export type InputButtonProps = {
    clickAction: () => void,
    caption: string,
    classNames: string,
    tooltipText?: string,
    tooltipToggle?: ((enabled: boolean, text: string) => void),
}
export default function InputButton(props: InputButtonProps) {
    function tooltipEnable() {
        if ( props.tooltipToggle !== undefined && props.tooltipText !== undefined ) {
            props.tooltipToggle(true, props.tooltipText)
        }
    }

    function tooltipDisable() {
        if ( props.tooltipToggle !== undefined && props.tooltipText !== undefined ) {
            props.tooltipToggle(false, props.tooltipText)
        }
    }
    return (
        <button
            className={"flex-table-cell "+props.classNames}
            onClick={props.clickAction}
            onMouseEnter={tooltipEnable}
            onMouseLeave={tooltipDisable}
        >
            {props.caption}
        </button>
    )
}
