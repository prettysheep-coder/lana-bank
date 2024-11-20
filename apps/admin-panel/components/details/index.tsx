import * as React from "react"
import { cva, type VariantProps } from "class-variance-authority"

import { cn } from "@/lib/utils"

const detailsGroupVariants = cva("", {
  variants: {
    layout: {
      vertical: "grid gap-6",
      horizontal: "flex flex-col",
    },
  },
  defaultVariants: {
    layout: "vertical",
  },
})

type LayoutType = NonNullable<VariantProps<typeof detailsGroupVariants>["layout"]>

interface DetailsGroupProps
  extends Omit<VariantProps<typeof detailsGroupVariants>, "layout"> {
  children: React.ReactNode
  className?: string
  layout?: LayoutType
}

interface DetailItemProps {
  label: React.ReactNode
  value: React.ReactNode
  className?: string
  onClick?: (() => void) | null
  hover?: boolean
  keyTestId?: string
  valueTestId?: string
  keyClassName?: string
}

const DetailsGroupContext = React.createContext<LayoutType>("vertical")

const DetailsGroup = ({
  children,
  layout = "vertical",
  className,
}: DetailsGroupProps) => {
  const childrenArray = React.Children.toArray(children)
  const totalItems = childrenArray.length
  const columns = totalItems > 2 ? 4 : 2

  return (
    <DetailsGroupContext.Provider value={layout}>
      <div
        className={cn(
          detailsGroupVariants({ layout }),
          layout === "vertical" && `grid-cols-${columns}`,
          className,
        )}
      >
        {childrenArray}
      </div>
    </DetailsGroupContext.Provider>
  )
}

const DetailItem = ({
  label,
  value,
  className,
  onClick = null,
  hover = false,
  keyTestId,
  valueTestId,
  keyClassName,
}: DetailItemProps) => {
  const layout = React.useContext(DetailsGroupContext)

  const styles = {
    container: cn(
      "rounded-md font-semibold flex-wrap",
      layout === "vertical"
        ? "flex flex-col justify-between"
        : "flex justify-between items-center p-1",
      (hover || onClick) && "hover:bg-secondary",
      className,
    ),
    label: cn(
      "text-muted-foreground",
      layout === "vertical" ? "text-sm" : "font-normal",
      keyClassName,
    ),
    value: cn("text-md", layout === "vertical" ? "" : ""),
  }

  return (
    <div
      className={styles.container}
      onClick={onClick || undefined}
      data-testid={keyTestId}
    >
      <div className={styles.label}>{label}</div>
      <div className={styles.value} data-testid={valueTestId}>
        {value}
      </div>
    </div>
  )
}

export { DetailItem, DetailsGroup, type LayoutType }
