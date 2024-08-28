"use client"

import { createContext, useCallback, useContext, useEffect, useState } from "react"

type PriceContextType = {
  price: number
  satsToCents: null | ((sats: number) => number)
}

const initialPriceValues: PriceContextType = {
  price: 60000_00, // 1BTC = 6000000 cents
  satsToCents: null,
}

const PriceContext = createContext(initialPriceValues)
const PriceProvider: React.FC<React.PropsWithChildren> = ({ children }) => {
  const [price, setPrice] = useState(initialPriceValues.price)

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
    }, 1000)

    return () => clearInterval(intervalId)
  }, [])

  const satsToCents = useCallback(
    (sats: number) => sats * (1 / 100000000) * price,
    [price],
  )

  return (
    <PriceContext.Provider value={{ price, satsToCents }}>
      {children}
    </PriceContext.Provider>
  )
}

const usePrice = () => useContext(PriceContext)

export { PriceProvider, usePrice }
