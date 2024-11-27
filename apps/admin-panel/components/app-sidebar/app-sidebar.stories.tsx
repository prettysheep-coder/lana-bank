import type { Meta } from "@storybook/react"
import { ApolloError } from "@apollo/client"
import { MockedProvider } from "@apollo/client/testing"

import { AppSidebar } from "./"

import { SidebarProvider } from "@/ui/sidebar"

import {
  Role,
  AvatarDocument,
  GetRealtimePriceUpdatesDocument,
} from "@/lib/graphql/generated"
import { mockUser } from "@/lib/graphql/generated/mocks"
import faker from "@/.storybook/faker"

interface SidebarStoryProps {
  userEmail: string
  userRoles: Role[]
  btcPriceUSD: number
  showError: boolean
}

const SidebarWithProviders = (props: SidebarStoryProps) => (
  <div className="h-screen">
    <MockedProvider mocks={createMocks(props)} addTypename={false}>
      <SidebarProvider>
        <AppSidebar />
      </SidebarProvider>
    </MockedProvider>
  </div>
)

const DEFAULT_ARGS: SidebarStoryProps = {
  userEmail: faker.internet.email(),
  userRoles: [Role.Admin],
  btcPriceUSD: faker.number.int({ min: 20, max: 1000 }),
  showError: false,
}

const createMocks = (args: SidebarStoryProps) => {
  if (args.showError) {
    return [
      {
        request: { query: AvatarDocument },
        error: new ApolloError({ errorMessage: faker.lorem.sentence() }),
      },
      {
        request: { query: GetRealtimePriceUpdatesDocument },
        error: new ApolloError({ errorMessage: faker.lorem.sentence() }),
      },
    ]
  }

  return [
    {
      request: { query: AvatarDocument },
      result: {
        data: {
          me: {
            user: mockUser({
              email: args.userEmail,
              roles: args.userRoles,
            }),
          },
        },
      },
    },
    {
      request: { query: GetRealtimePriceUpdatesDocument },
      result: {
        data: {
          realtimePrice: {
            usdCentsPerBtc: args.btcPriceUSD * 100,
          },
        },
      },
    },
  ]
}

const meta = {
  title: "Components/AppSidebar",
  component: SidebarWithProviders,
  parameters: {
    layout: "fullscreen",
    nextjs: { appDirectory: true },
    backgrounds: {
      default: "light",
    },
  },
  argTypes: {
    userEmail: {
      control: "text",
      description: "User's email address",
    },
    userRoles: {
      control: "multi-select",
      options: Object.values(Role),
      description: "User's roles",
    },
    btcPriceUSD: {
      control: { type: "number", min: 1000, max: 100000, step: 1000 },
      description: "Bitcoin price in USD",
    },
    showError: {
      control: "boolean",
      description: "Show error state",
    },
  },
  tags: ["autodocs"],
} satisfies Meta<typeof SidebarWithProviders>

export default meta

export const Default = {
  args: DEFAULT_ARGS,
}

export const Error = {
  args: {
    ...DEFAULT_ARGS,
    showError: true,
  },
}
