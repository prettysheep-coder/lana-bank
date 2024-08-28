"use client"

import { createContext, useCallback, useContext, useEffect, useState } from "react"

import { formatCurrency } from "../utils"

type PriceContextType = {
  price: number | null
  livePriceData: string
  satsToCents: null | ((sats: number) => number)
  centsToSats: null | ((cents: number) => number)
}

const initialPriceValues: PriceContextType = {
  price: null,
  satsToCents: null,
  centsToSats: null,
  livePriceData: "",
}

const PriceContext = createContext(initialPriceValues)
const PriceProvider: React.FC<React.PropsWithChildren> = ({ children }) => {
  const [price, setPrice] = useState(60000_00) // 1BTC = 6000000 cents

  /* THIS ENTIRE USEEFFECT YIELDS DUMMY CHANGING VALUES TO PRICE
   * TODO: Remove this and query the price from the server
   */
  useEffect(() => {
    const intervalId = setInterval(() => {
      // Update the price by a random amount every second
      // Let's assume the price changes by +/- 0.1% to 0.5% each second
      const randomPercentage = 0.001 + Math.random() * 0.004 // random percentage change
      const changeFactor =
        Math.random() > 0.5 ? 1 + randomPercentage : 1 - randomPercentage
      setPrice((prevPrice) => prevPrice * changeFactor)
    }, 60000)

    return () => clearInterval(intervalId)
  }, [])

  const satsToCents = useCallback(
    (sats: number) => sats * (1 / 100000000) * price,
    [price],
  )
  const centsToSats = useCallback((cents: number) => (cents / price) * 100000000, [price])

  const livePriceData = `BTC/USD: ${formatCurrency({ amount: price / 100, currency: "USD" })}`

  return (
    <PriceContext.Provider value={{ price, satsToCents, centsToSats, livePriceData }}>
      {children}
    </PriceContext.Provider>
  )
}

const usePrice = () => useContext(PriceContext)

export { PriceProvider, usePrice }
