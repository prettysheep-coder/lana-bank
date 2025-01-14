"use client"
import React from "react"

import {
  Command,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator,
} from "@/ui/command"

import {
  navDashboardItems,
  navLoansItems,
  navCustomersItems,
  navTransactionItems,
  navAdminItems,
  navFinanceItems,
} from "@/components/app-sidebar/nav-items"

import { useRouter } from "next/navigation"

const CommandMenu = () => {
  const [open, setOpen] = React.useState(false)
  const [pages, setPages] = React.useState<"main" | "navigation">("main")
  const router = useRouter()

  React.useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === "k" && (e.metaKey || e.ctrlKey)) {
        e.preventDefault()
        setPages("main")
        setOpen((open) => !open)
      }
      if (e.key === "n" && (e.metaKey || e.ctrlKey)) {
        e.preventDefault()
        setPages("navigation")
        setOpen((open) => !open)
      }
    }

    document.addEventListener("keydown", down)
    return () => document.removeEventListener("keydown", down)
  }, [])

  React.useEffect(() => {
    if (!open) {
      setPages("main")
    }
  }, [open])

  const allNavItems = [
    ...navDashboardItems,
    ...navLoansItems,
    ...navCustomersItems,
    ...navTransactionItems,
    ...navAdminItems,
    ...navFinanceItems,
  ]

  return (
    <CommandDialog open={open} onOpenChange={setOpen}>
      <Command className="rounded-lg border shadow-md">
        <CommandInput
          placeholder={
            pages === "navigation" ? "Search navigation..." : "What do you need?"
          }
        />
        <CommandList>
          <CommandEmpty>No results found.</CommandEmpty>

          {pages === "main" ? (
            <>
              <CommandGroup
                heading={
                  <div className="flex items-center justify-between">
                    <span>Navigation</span>
                    <kbd className="ml-auto pointer-events-none inline-flex h-5 select-none items-center gap-1 rounded border bg-muted px-1.5 font-mono text-[10px] font-medium text-muted-foreground opacity-100">
                      <span className="text-xs">Ctrl +</span>N
                    </kbd>
                  </div>
                }
              >
                {allNavItems.map((item) => (
                  <CommandItem
                    key={item.url}
                    onSelect={() => {
                      window.location.href = item.url
                      setOpen(false)
                    }}
                    className="flex items-center gap-2"
                  >
                    <item.icon className="h-4 w-4" />
                    <span>{item.title}</span>
                  </CommandItem>
                ))}
              </CommandGroup>

              <CommandSeparator />

              <CommandGroup heading="General">
                <CommandItem className="flex items-center gap-2">
                  <span>Change Theme...</span>
                </CommandItem>
                <CommandItem className="flex items-center gap-2">
                  <span>Copy Current URL</span>
                </CommandItem>
              </CommandGroup>
            </>
          ) : (
            <CommandGroup heading="Navigation">
              {allNavItems.map((item) => (
                <CommandItem
                  key={item.url}
                  onSelect={() => {
                    setOpen(false)
                    router.push(item.url)
                  }}
                  className="flex items-center gap-2"
                >
                  <item.icon className="h-4 w-4" />
                  <span>{item.title}</span>
                </CommandItem>
              ))}
            </CommandGroup>
          )}
        </CommandList>
      </Command>
    </CommandDialog>
  )
}

export { CommandMenu }
