"use client"

import React from 'react'
import { useState, useEffect } from 'react'

import TextField from '@/components/textField'
import Title from '@/components/titleBar'
import InputButton from '@/components/inputButton'
import ToolTip from '@/components/tooltip'
import { CheckResult } from '@/components/textField'
import { Uuid } from '@/components/types/id'

import { config, apiURL } from '@/components/config'
import { notifier } from '@/app/notifier'

export default function Home() {
  const [result, setResult] = useState<CheckResult>({
    result: true,
    kind: "notYetAvailable",
    message: "No checking had been conducting yet.",
  })

  const tooltipClassEnabled = "opacity-75 blur-none drop-shadow-[0_0_0.4rem_#ffffff99]"
  const tooltipClassDisabled = "opacity-0 blur-[6px]"

  const [existingBoards, setExistingBoards] = useState<Array<[string, string]>>([])
  const [tooltipText, setTooltipText] = useState<string>("")
  const [tooltipClassNames, setTooltipClassNames] = useState<string>(tooltipClassDisabled)

  if ( config.simulated_failure_factor != 0 ) {
    console.log("Simulating failure at a non-zero rate. This interface does not currently support failure simulation; some requests may fail.")
  }

  async function getExistingBoards() {
    const response = await fetch(apiURL+'/list')
    const json = await response.json()
    if ( response.ok ) {
      setExistingBoards(json.map((board: any) => [ board.uuid, `Game ${board.uuid.substring(0,4)}: ${board.size[0]}x${board.size[1]}, ${board.active ? 'active':'completed'}` ]))
    } else {
      notifier.alert(json.message, {labels:{alert: json.error}})
    }
  }

  async function checkUUID(uuid: string): Promise<CheckResult> {
    return Uuid.isValid(uuid) ? {
      result: true,
      kind: null,
      message: `${uuid} is valid.`,
    } : {
      result: false,
      kind: "InvalidUuid",
      message: `${uuid} is not a valid UUID.`,
    }
  }

  async function goToBoard(response: Response): Promise<CheckResult> {
    const json = await response.json()
    // TODO: This is a hack. We should find a way to find the UUID neatly.
    window.location.href = `/board/${json.uuid ?? json.game.uuid}`

    if ( response.ok ) {
      return {
        result: true,
        kind: null,
        message: null,
      }
    } else {
      return {
        result: false,
        kind: json.error,
        message: json.message,
      }
    }
  }

  async function goToExistingBoard(uuid: string): Promise<CheckResult> {
    const response = await fetch(apiURL+`/status/${uuid}`)

    return await goToBoard(response)
  }

  async function goToNewBoard(): Promise<CheckResult> {
    const response = await fetch(apiURL+'/new')

    return await goToBoard(response)
  }

  function tooltipToggle(enabled: boolean, text: string) {
    setTooltipText(text)
    setTooltipClassNames(enabled ? tooltipClassEnabled : tooltipClassDisabled)
  }

  useEffect(() => {
    getExistingBoards().catch(console.error)
  }, [])

  return (
    <main className="flex-table grid-cols-2">
        <Title
          title="battleship"
          classNames="
            col-span-2
            font-serif
            tracking-[1.5rem]
            text-4xl
            font-light
            text-slate-400
            text-center
            pb-12
            blur-[2px]
            hover:blur-none
            transition
            ease-in-out
            duration-1000
            translate-x-[0.9rem]
            hover:drop-shadow-[0_0_1.4rem_#ffffffff]
            bg-image-ship-battleship
            bg-contain
            bg-no-repeat
            bg-top
        "
          tooltipText='A pointless battleship game against the computer for demonstration only.'
          tooltipToggle={tooltipToggle}
        />
        <InputButton classNames='
            cursor-pointer
            rounded-l-full

            h-16
            w-96

            bg-gradient-to-r
            from-yellow-800
            to-red-600

            blur-[2px]
            hover:blur-none

            hover:from-yellow-700
            hover:to-red-500
            hover:text-white

            transition
            ease-in-out
            duration-1000

            text-black
            text-center
            font-serif
            font-light
          '
          caption="new board"
          clickAction={goToNewBoard}
          tooltipText='Create a new game with default settings: 10x10, 5 ships.'
          tooltipToggle={tooltipToggle}
        />
        <TextField
          typeAction={checkUUID}
          enterAction={goToExistingBoard}
          setResult={setResult}
          placeholderText="existing board UUID"
          defaults={existingBoards}
          classNames='
            rounded-r-full

            h-16
            w-96

            bg-gradient-to-r
            from-cyan-600
            to-blue-800

            transition
            ease-in-out
            duration-1000

            blur-[2px]
            hover:blur-none
            focus:blur-none

            text-white
            text-center
            font-mono
            font-semibold
            placeholder:text-black
            placeholder:font-light
            placeholder:font-serif
          '
          tooltipText='Enter UUID of an existing board you want to continue.'
          tooltipToggle={tooltipToggle}
        />
        <ToolTip text={tooltipText} classNames={`
            col-span-2
            text-center
            font-serif
            font-light
            text-slate-500
            text-sm

            transition
            ease-in-out
            duration-1000

            pt-8

            h-24

          ` + tooltipClassNames
        }/>
        </main>
  )
}
