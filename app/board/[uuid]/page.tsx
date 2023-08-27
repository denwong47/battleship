"use client"

import React from 'react'
import { useState, useEffect } from 'react'

import Head from 'next/head';

import { BoardStatus } from '@/components/types/boardStatus'
import { Strike } from '@/components/types/strike'
import { apiURL } from '@/components/config'
import { notifier } from '@/app/notifier'
import BoardDisplay from '@/components/boardDisplay'
import IntelDisplay from '@/components/intelDisplay';

export default function Board({ params }: { params: { uuid: string }}) {
  const [boardState, setBoardState] = useState<BoardStatus | null>(null)
  const [strikes, setStrikes] = useState<Strike[]>([])

  async function fetch_board_state(): Promise<BoardStatus | null> {
    let [responseBoard, responseStrikes] = await Promise.all([
      fetch(`${apiURL}/status/${params.uuid}`),
      fetch(`${apiURL}/list_strikes/${params.uuid}`)
    ])

    if (responseBoard.ok) {
      let status = BoardStatus.fromJson(await responseBoard.json(), setBoardState)
      let strikes = (await responseStrikes.json()).map((strike: any) => Strike.fromJson(strike))

      setBoardState(status)
      setStrikes(strikes)

      return status
    } else {
      return null
    }
  }

  function addStrike(strike: Strike) {
    setStrikes([...strikes, strike])
  }

  async function backToLobby() {
    notifier.confirm(
      "Are you sure you want to leave this game?",
      () => { window.location.href = '/' },
      () => {},
      {
        labels: {
          confirm: 'Leave game'
        }
      }
    )
  }

  async function syncBoardState() {
    await notifier.async(
      fetch_board_state(),
      () => notifier.success("Board state synced.", {labels: {success: "\u{1F4E6} Up to date"}}),
      () => notifier.alert("Board state could not be synced.", {labels: {alert: "\u{1F6AB} Sync failed"}}),
      "Syncing board state...",
      {labels: {async: "\u{231B} Syncing"}}
    )
  }

  useEffect(() => {
    fetch_board_state()
  }, [])

  return (
    <main className='flex flex-col-reverse grid-cols-1 md:flex-row md:grid-cols-2 w-full [&>*]:p-4'>
      <Head><title>Battleship game</title></Head>
      <IntelDisplay boardState={boardState} backToLobby={backToLobby} syncBoardState={syncBoardState} />
      <BoardDisplay boardState={boardState} strikes={strikes} addStrike={addStrike} />
    </main>
  )
}
