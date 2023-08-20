"use client"

import React from 'react'
import { useState, useEffect } from 'react'

import TextField from '@/components/textField'
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

  const [existingBoards, setExistingBoards] = useState<Array<[string, string]>>([])

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

  useEffect(() => {
    getExistingBoards().catch(console.error)
  }, [])

  return (
    <main className="flex-table grid-cols-2">
        <input type="button" className='
            cursor-pointer
            rounded-l-full

            h-16
            w-96

            bg-gradient-to-r
            from-yellow-800
            to-red-600

            hover:from-yellow-700
            hover:to-red-500
            hover:text-white

            transition
            ease-in-out
            duration-300

            text-slate-400
            text-center
            font-mono
            font-semibold
            placeholder:text-slate-400
            placeholder:font-light
          ' value="new board" onClick={goToNewBoard} />
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
            duration-300

            text-white
            text-center
            font-mono
            font-semibold
            placeholder:text-slate-400
            placeholder:font-light
          '
        />
        </main>
  )
}
