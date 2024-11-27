"use client"

import {
  Home,
  Mouse,
  Users,
  LayoutGrid,
  ClipboardList,
  FileText,
  UserCircle,
  ArrowDownCircle,
  ArrowUpCircle,
  FileBarChart,
  Globe,
  PieChart,
  DollarSign,
  LineChart,
  LogOut,
  ChevronsUpDown,
  Users2,
  GanttChart,
} from "lucide-react"

import Link from "next/link"
import { signOut } from "next-auth/react"
import { usePathname } from "next/navigation"
import { gql } from "@apollo/client"

import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarGroup,
  SidebarGroupLabel,
} from "@/ui/sidebar"

import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/ui/dropdown-menu"

import { useAvatarQuery, useGetRealtimePriceUpdatesQuery } from "@/lib/graphql/generated"
import { currencyConverter } from "@/lib/utils"
import { Skeleton } from "@/ui/skeleton"
import { Badge } from "@/ui/badge"

import { ID } from "@/components/id"

gql`
  query Avatar {
    me {
      user {
        userId
        email
        roles
      }
    }
  }
`

const navDashboardItems = [
  { title: "Dashboard", url: "/dashboard", icon: Home },
  { title: "Actions", url: "/actions", icon: Mouse },
]

const navMainItems = [
  { title: "Customers", url: "/customers", icon: Users },
  { title: "Credit Facilities", url: "/credit-facilities", icon: LayoutGrid },
  { title: "Disbursals", url: "/disbursals", icon: ClipboardList },
  { title: "Terms Templates", url: "/terms-templates", icon: FileText },
  { title: "Users", url: "/users", icon: UserCircle },
  { title: "Committees", url: "/committees", icon: Users2 },
  { title: "Policies", url: "/policies", icon: GanttChart },
]

const navTransactionItems = [
  { title: "Deposits", url: "/deposits", icon: ArrowDownCircle },
  { title: "Withdrawals", url: "/withdrawals", icon: ArrowUpCircle },
]

const navFinanceItems = [
  { title: "Regulatory Reporting", url: "/regulatory-reporting", icon: FileBarChart },
  { title: "Chart of Accounts", url: "/chart-of-accounts", icon: Globe },
  { title: "Balance Sheet", url: "/balance-sheet", icon: PieChart },
  { title: "Profit & Loss", url: "/profit-and-loss", icon: DollarSign },
  { title: "Trial Balance", url: "/trial-balance", icon: LineChart },
]

function NavSection({ items, label }: { items: typeof navMainItems; label?: string }) {
  const pathname = usePathname()

  return (
    <SidebarGroup>
      {label && <SidebarGroupLabel>{label}</SidebarGroupLabel>}
      <SidebarMenu>
        {items.map((item) => {
          const Icon = item.icon
          const isActive = pathname?.startsWith(item.url)

          return (
            <SidebarMenuItem key={item.url}>
              <SidebarMenuButton asChild tooltip={item.title} isActive={isActive}>
                <Link href={item.url} prefetch={true}>
                  <Icon className="h-4 w-4" />
                  <span>{item.title}</span>
                </Link>
              </SidebarMenuButton>
            </SidebarMenuItem>
          )
        })}
      </SidebarMenu>
    </SidebarGroup>
  )
}
function UserBlock() {
  const { data, loading } = useAvatarQuery()

  if (loading) {
    return (
      <SidebarMenu>
        <SidebarMenuItem>
          <SidebarMenuButton size="lg">
            <Skeleton className="h-8 w-8 rounded-lg" />
            <div className="grid flex-1 gap-2">
              <Skeleton className="h-4 w-24" />
              <Skeleton className="h-3 w-32" />
            </div>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </SidebarMenu>
    )
  }

  if (!data?.me.user) return null
  const { email, roles, userId } = data.me.user
  const userName = email.split("@")[0]
  const initials = userName[0].toUpperCase()

  return (
    <SidebarMenu>
      <SidebarMenuItem>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <SidebarMenuButton size="lg">
              <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-primary text-primary-foreground">
                <span className="text-sm font-medium">{initials}</span>
              </div>
              <div className="grid flex-1 text-left text-sm leading-tight">
                <span className="truncate font-medium capitalize">{userName}</span>
                <span className="truncate text-xs text-muted-foreground">{email}</span>
              </div>
              <ChevronsUpDown className="ml-auto size-4 text-muted-foreground/70" />
            </SidebarMenuButton>
          </DropdownMenuTrigger>
          <DropdownMenuContent className="min-w-56" align="end" sideOffset={4}>
            <DropdownMenuLabel className="font-normal">
              <div className="flex flex-col gap-2 p-1">
                <div className="flex flex-wrap gap-1">
                  {roles.map((role) => (
                    <Badge key={role} variant="secondary" className="capitalize">
                      {role.toLowerCase()}
                    </Badge>
                  ))}
                </div>
                <div className="text-sm">{email}</div>
                <ID type="Your" id={userId} />
              </div>
            </DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuItem
              className="text-destructive focus:text-destructive"
              onClick={() => signOut()}
            >
              <LogOut className="mr-2 h-4 w-4" />
              Log out
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </SidebarMenuItem>
    </SidebarMenu>
  )
}

function MarketRate() {
  const { data, loading } = useGetRealtimePriceUpdatesQuery()
  const usdBtcRate = currencyConverter
    .centsToUsd(data?.realtimePrice?.usdCentsPerBtc || NaN)
    .toLocaleString()

  if (loading) return <Skeleton className="h-4 w-full py-2" />

  return (
    <div className="flex items-center px-2 py-2 gap-1 text-sm text-muted-foreground font-medium">
      <div>USD/BTC Market Rate: </div>
      <div>{String(usdBtcRate) === "NaN" ? "Not Available" : `$${usdBtcRate}`}</div>
    </div>
  )
}

export function AppSidebar({ ...props }: React.ComponentProps<typeof Sidebar>) {
  return (
    <Sidebar variant="inset" {...props}>
      <SidebarHeader>
        <UserBlock />
      </SidebarHeader>

      <SidebarContent className="mt-4">
        <NavSection items={navDashboardItems} />
        <NavSection items={navMainItems} />
        <NavSection items={navTransactionItems} label="Transactions" />
        <NavSection items={navFinanceItems} label="Financial Reports" />
      </SidebarContent>

      <SidebarFooter>
        <MarketRate />
      </SidebarFooter>
    </Sidebar>
  )
}
