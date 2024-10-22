import { Inter as GoogleFont_Inter } from "next/font/google"
import LocalFont from "next/font/local"

export const Helvetica = LocalFont({
  src: [
    {
      path: "./helvetica-now/HelveticaNowDisplay-Light.woff2",
      weight: "300",
      style: "normal",
    },
    {
      path: "./helvetica-now/HelveticaNowDisplay-Regular.woff2",
      weight: "400",
      style: "normal",
    },
    {
      path: "./helvetica-now/HelveticaNowDisplay-Medium.woff2",
      weight: "500",
      style: "normal",
    },
  ],
  variable: "--font-helvetica",
})

export const Inter = GoogleFont_Inter({
  weight: "500",
  subsets: ["latin"],
  variable: "--font-inter",
})
