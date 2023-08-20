import React from 'react'

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
}

function notifyResult(result: CheckResult) {
    if ( !result.result ) {
        notifier.alert(result.message, {labels:{alert: result.kind}})
    }
}

export default function TextField(props: TextFieldProps) {
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

    return (
        <main>
        <input type="text" className={"flex-table-cell "+props.classNames} placeholder={props.placeholderText} onKeyDown={checkAction} list="dropdown-options"/>
        <datalist id="dropdown-options">
        {
            props.defaults.map(([value, text]) => <option key={value} value={value}>{text}</option>)
        }
        </datalist>
        </main>
    )
}
