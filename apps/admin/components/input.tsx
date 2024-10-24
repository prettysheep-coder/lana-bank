"use client"

import { HTMLInputTypeAttribute, useState } from "react"

type InputProps = {
  label: string
  type: HTMLInputTypeAttribute
  defaultValue?: string
  onChange?: (text: string) => void
  name?: string
  placeholder?: string
  autofocus?: boolean
  required?: boolean

  // If type is 'number' and numeric is set, the displayed number will contain commas for thousands separators
  numeric?: boolean
}

const Input: React.FC<InputProps> = ({
  label,
  type,
  // eslint-disable-next-line no-empty-function
  onChange = () => {},
  defaultValue = "",
  placeholder = "",
  name,
  numeric = false,
  autofocus = false,
  required = false,
}) => {
  const [_displayValue, setDisplayValue] = useState(defaultValue)
  let displayValue = _displayValue

  const isNumeric = numeric && type === "number"

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    let value = e.target.value

    if (isNumeric) {
      value = value.replaceAll(",", "").replace(/\D/g, "")
    }

    setDisplayValue(value)
    onChange(value)
  }

  if (isNumeric && _displayValue !== "") {
    displayValue = Number(_displayValue).toLocaleString("en-US")
  }

  return (
    <div className="flex flex-col space-y-1 w-full">
      <label className="text-medium" htmlFor={name}>
        {label}
      </label>
      <input
        className="dark:bg-secondary focus:ring-primary focus:ring-1 focus:outline-none border border-primary-bg rounded-md p-2"
        type={isNumeric ? "text" : type}
        value={displayValue}
        onChange={handleChange}
        id={name}
        name={name}
        placeholder={placeholder}
        autoFocus={autofocus}
        required={required}
      />
    </div>
  )
}

export default Input
