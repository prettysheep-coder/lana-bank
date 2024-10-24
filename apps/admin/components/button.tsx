"use client"

import { ButtonHTMLAttributes } from "react"

import { Button as MTButton } from "@material-tailwind/react"

type ButtonProps = {
  title: string
  type?: ButtonHTMLAttributes<HTMLButtonElement>["type"]
  onClick?: () => void
  className?: string
}
const Button: React.FC<ButtonProps> = ({
  title,
  type = "button",
  className = "",
  // eslint-disable-next-line no-empty-function
  onClick = () => {},
}) => {
  return (
    <MTButton
      className={`bg-secondary dark:text-grey-0 ${className}`}
      type={type}
      placeholder={undefined}
      onPointerEnterCapture={undefined}
      onPointerLeaveCapture={undefined}
      onClick={onClick}
    >
      {title}
    </MTButton>
  )
}

export default Button
