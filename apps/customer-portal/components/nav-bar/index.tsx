"use client"

import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "@lana/web/ui/dropdown-menu"
import { Avatar, AvatarFallback } from "@lana/web/ui/avatar"
import { Laptop, LogOut, Moon, Settings, Sun, User } from "lucide-react"
import { useTheme } from "next-themes"

import { Badge } from "@lana/web/ui/badge"

import { KycLevel } from "@/lib/graphql/generated"

import { useLogout } from "@/hooks/use-logout"

interface MenuItem {
  label: string
  icon?: React.ReactNode
  onClick?: () => void
  href?: string
}

function NavBar() {
  const { setTheme } = useTheme()
  const { logout } = useLogout()

  const menuItems: MenuItem[] = [
    {
      label: "Profile",
      icon: <User className="mr-2 h-4 w-4" />,
      href: "/profile",
    },
    {
      label: "Settings",
      icon: <Settings className="mr-2 h-4 w-4" />,
      href: "/settings",
    },
    {
      label: "Logout",
      icon: <LogOut className="mr-2 h-4 w-4" />,
      onClick: () => logout(),
    },
  ]

  const handleMenuItemClick = (item: MenuItem) => {
    if (item.onClick) {
      item.onClick()
    }
    if (item.href) {
      console.log(`Navigate to: ${item.href}`)
    }
  }

  return (
    <nav>
      <div className="max-w-7xl mx-auto px-4">
        <div className="flex items-center justify-between h-16">
          <div className="flex items-center gap-8">
            <div className="text-2xl font-semibold">Lana</div>
          </div>
          <div className="flex gap-2">
            <KYCBadge level={KycLevel.NotKyced} />
            <DropdownMenu>
              <DropdownMenuTrigger className="focus:outline-none">
                <Avatar className="h-9 w-9 cursor-pointer rounded-md [&>span]:rounded-md">
                  <AvatarFallback>LA</AvatarFallback>
                </Avatar>
              </DropdownMenuTrigger>

              <DropdownMenuContent align="end" className="w-48">
                {menuItems.map((item, index) => (
                  <DropdownMenuItem
                    key={index}
                    className="flex items-center cursor-pointer"
                    onClick={() => handleMenuItemClick(item)}
                  >
                    {item.icon}
                    <span>{item.label}</span>
                  </DropdownMenuItem>
                ))}

                <DropdownMenuSeparator />
                <DropdownMenuSub>
                  <DropdownMenuSubTrigger className="cursor-pointer">
                    <Sun className="mr-2 h-4 w-4" />
                    <span>Theme</span>
                  </DropdownMenuSubTrigger>
                  <DropdownMenuSubContent>
                    <DropdownMenuItem onClick={() => setTheme("light")}>
                      <Sun className="mr-2 h-4 w-4" />
                      <span>Light</span>
                    </DropdownMenuItem>
                    <DropdownMenuItem onClick={() => setTheme("dark")}>
                      <Moon className="mr-2 h-4 w-4" />
                      <span>Dark</span>
                    </DropdownMenuItem>
                    <DropdownMenuItem onClick={() => setTheme("system")}>
                      <Laptop className="mr-2 h-4 w-4" />
                      <span>System</span>
                    </DropdownMenuItem>
                  </DropdownMenuSubContent>
                </DropdownMenuSub>
              </DropdownMenuContent>
            </DropdownMenu>
          </div>
        </div>
      </div>
    </nav>
  )
}

export default NavBar

import { cn } from "@/lib/utils"

interface KYCBadgeProps {
  level: KycLevel
  className?: string
}

const levelStyles = {
  [KycLevel.NotKyced]:
    "border-destructive/50 text-destructive bg-destructive/10 hover:bg-destructive/20",
  [KycLevel.Basic]:
    "border-orange-500/50 text-orange-700 dark:text-orange-400 bg-orange-100/50 dark:bg-orange-900/20 hover:bg-orange-100 dark:hover:bg-orange-900/30",
  [KycLevel.Advanced]:
    "border-emerald-500/50 text-emerald-700 dark:text-emerald-400 bg-emerald-100/50 dark:bg-emerald-900/20 hover:bg-emerald-100 dark:hover:bg-emerald-900/30",
}

const levelLabels = {
  [KycLevel.NotKyced]: "Not KYCed",
  [KycLevel.Basic]: "Basic KYC",
  [KycLevel.Advanced]: "Advanced KYC",
}

export function KYCBadge({ level, className }: KYCBadgeProps) {
  return (
    <Badge
      variant="outline"
      className={cn(
        "rounded-lg font-semibold transition-colors duration-200",
        levelStyles[level],
        className,
      )}
    >
      {levelLabels[level]}
    </Badge>
  )
}
