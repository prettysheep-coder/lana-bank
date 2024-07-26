import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

const CENTS_PER_USD = 100
const SATOSHI_PER_BTC = 100000000

export const currencyConverter = {
  centsToUsd: (cents: number) => {
    return Number((cents / CENTS_PER_USD).toFixed(2))
  },

  btcToSatoshi: (btc: number) => {
    return Number((btc * SATOSHI_PER_BTC).toFixed(0))
  },

  satoshiToBtc: (satoshi: number) => {
    return satoshi / SATOSHI_PER_BTC
  },

  usdToCents: (usd: number) => {
    return Number((usd * CENTS_PER_USD).toFixed(0))
  },
}

export function formatCurrency({
  amount,
  currency,
}: {
  amount: number
  currency: string
}) {
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency,
  }).format(amount)
}

export const formatDate = (isoDateString: string): string => {
  const date = new Date(isoDateString)
  const options: Intl.DateTimeFormatOptions = {
    year: "numeric",
    month: "long",
    day: "numeric",
  }

  const formattedDate = date.toLocaleDateString("en-US", options)
  const formattedTime = date
    .toLocaleTimeString("en-US", {
      hour: "2-digit",
      minute: "2-digit",
      hour12: true,
    })
    .toUpperCase()

  return `${formattedDate}, ${formattedTime}`
}
