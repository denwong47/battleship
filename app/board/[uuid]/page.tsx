"use client"

import React from 'react'
import { useState, useEffect } from 'react'

import Head from 'next/head';

import { BoardStatus } from '@/components/types/boardStatus'
import { Strike } from '@/components/types/strike'
import BoardDisplay from '@/components/boardDisplay'
import { apiURL } from '@/components/config'

export default function Board({ params }: { params: { uuid: string }}) {
  const [boardState, setBoardState] = useState<BoardStatus | null>(null)
  const [strikes, setStrikes] = useState<Strike[]>([])

  async function fetch_board_state(): Promise<BoardStatus | null> {
    let [responseBoard, responseStrikes] = await Promise.all([
      fetch(`${apiURL}/status/${params.uuid}`),
      fetch(`${apiURL}/list_strikes/${params.uuid}`)
    ])

    if (responseBoard.ok) {
      let status = BoardStatus.fromJson(await responseBoard.json())
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

  function freezeBoard() {
    if (boardState !== null) {
      setBoardState({
        ...boardState,
        active: false,
      })
    } else {
      console.error("Attempted to freeze board when boardState was `null`.")
    }
  }

  // TODO: Use another way to trigger this effect
  useEffect(() => {
    fetch_board_state()
  }, [])

  return (
    <main>
      <Head><title>Battleship game</title></Head>
      <BoardDisplay boardState={boardState} strikes={strikes} addStrike={addStrike} freezeBoard={freezeBoard} />
    </main>
  )
}
