"use client"

import { ButtonHTMLAttributes } from "react"

import { Button as MTButton } from "@material-tailwind/react"

type ButtonProps = {
  title: string
  type: ButtonHTMLAttributes<HTMLButtonElement>["type"]
}
const Button: React.FC<ButtonProps> = ({ title, type = "button" }) => {
  return (
    <MTButton
      className="bg-secondary"
      type={type}
      placeholder={undefined}
      onPointerEnterCapture={undefined}
      onPointerLeaveCapture={undefined}
    >
      {title}
    </MTButton>
  )
}

export default Button
