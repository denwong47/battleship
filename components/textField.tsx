import React from 'react'
import { useState } from 'react'
import { notifier } from '@/app/notifier'

export type CheckResult = {
    result: boolean,
    kind: string | null,
    message: string | null,
}

export type TextFieldProps = {
    typeAction: (text: string) => Promise<CheckResult>,
    enterAction: (text: string) => Promise<CheckResult>,
    setResult: (result: CheckResult) => void,
    placeholderText: string,
    defaults: Array<[string, string]>,
    classNames: string,
    tooltipText?: string,
    tooltipToggle?: ((enabled: boolean, text: string) => void),
}

function notifyResult(result: CheckResult) {
    if ( !result.result ) {
        notifier.alert(result.message, {labels:{alert: result.kind}})
    }
}

export default function TextField(props: TextFieldProps) {
    const [focusCount, setFocusCount] = useState(0)

    async function checkAction(e: React.KeyboardEvent<HTMLInputElement>) {
        let text = (e.target as HTMLInputElement).value

        let typeResult = await props.typeAction(text)

        if ( e.key === 'Enter' ) {
            if ( !typeResult.result ) {
                notifyResult(typeResult)
                props.setResult(typeResult)
                return
            }

            let enterResult = await props.enterAction(text);

            notifyResult(enterResult)
            props.setResult(enterResult)
            return
        } else if ( !typeResult.result ) {
            props.setResult(typeResult)
            return
        }


        return
    }

    function modifyFocusCount(count: number) {
        const newFocusCount = focusCount+count
        setFocusCount(newFocusCount)
        if ( newFocusCount === 0 ) {
            tooltipDisable()
        } else {
            tooltipEnable()
        }
    }

    function addFocusCount() {
        modifyFocusCount(1)
    }

    function subFocusCount() {
        modifyFocusCount(-1)
    }

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
        <main>
        <input
            type="text"
            className={"flex-table-cell "+props.classNames}
            placeholder={props.placeholderText}
            list="dropdown-options"

            onKeyDown={checkAction}
            onFocus={addFocusCount}
            onBlur={subFocusCount}
            onMouseEnter={addFocusCount}
            onMouseLeave={subFocusCount}
        />
        <datalist id="dropdown-options">
        {
            props.defaults.map(([value, text]) => <option key={value} value={value}>{text}</option>)
        }
        </datalist>
        </main>
    )
}
