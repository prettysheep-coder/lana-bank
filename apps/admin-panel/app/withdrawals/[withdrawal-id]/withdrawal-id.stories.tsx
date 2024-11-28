import type { Meta, StoryObj } from "@storybook/react"
import { MockedProvider } from "@apollo/client/testing"

import WithdrawalPage from "./page"

import faker from "@/.storybook/faker"

import { GetWithdrawalDetailsDocument } from "@/lib/graphql/generated"
import { mockWithdrawal } from "@/lib/graphql/generated/mocks"

const createMocks = (withdrawalId: string) => [
  {
    request: {
      query: GetWithdrawalDetailsDocument,
      variables: { id: withdrawalId },
    },
    result: {
      data: {
        withdrawal: mockWithdrawal(),
      },
    },
  },
]

const WithdrawalStory = () => {
  const withdrawalId = faker.string.uuid()
  const mocks = createMocks(withdrawalId)
  return (
    <MockedProvider mocks={mocks} addTypename={false}>
      <WithdrawalPage params={{ "withdrawal-id": withdrawalId }} />
    </MockedProvider>
  )
}

const meta = {
  title: "Pages/Withdrawals/Withdrawal/Default",
  component: WithdrawalStory,
  parameters: {
    layout: "fullscreen",
    nextjs: {
      appDirectory: true,
    },
  },
} satisfies Meta<typeof WithdrawalPage>

export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = {}
