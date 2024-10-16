import React, { ReactNode } from "react"

import { IoArrowBack } from "react-icons/io5"

import Link from "next/link"

import { Button } from "../primitive/button"

import { cn } from "@/lib/utils"

interface PageHeadingProps {
  children: ReactNode
  className?: string
  backLink?: string
}

const PageHeading = ({ children, className, backLink }: PageHeadingProps) => {
  return (
    <>
      <title>{children}</title>
      <h1
        className={cn(
          "scroll-m-20 text-3xl font-semibold tracking-tight first:mt-0 mb-8 flex items-center gap-2",
          className,
        )}
      >
        {backLink && (
          <Link href={backLink}>
            <Button variant="ghost" className="p-1.5">
              <IoArrowBack className="w-7 h-7" />
            </Button>
          </Link>
        )}
        {children}
      </h1>
    </>
  )
}

export { PageHeading }
