export interface TitleProps {
    title: string;
    classNames: string;
    tooltipText?: string;
    tooltipToggle?: ((enabled: boolean, text: string) => void);
}

export default function Title(props: TitleProps) {

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
        <div className={props.classNames} onMouseEnter={tooltipEnable} onMouseLeave={tooltipDisable}>
          {props.title}
        </div>
    )
}
