import type { Meta, StoryObj } from "@storybook/react"
import { MockedProvider } from "@apollo/client/testing"

import TermPage from "./page"

import { TermsTemplatesDocument } from "@/lib/graphql/generated"
import { mockTermsTemplate } from "@/lib/graphql/generated/mocks"

const templateNames = [
  "High Risk",
  "High Risk",
  "Medium Risk",
  "Medium Risk",
  "Preferred Customer",
  "Preferred Customer",
  "Prime Customer",
  "Prime Customer",
  "Institutional",
  "Institutional",
]

const createRandomTermsTemplates = () => {
  return templateNames.map((name) =>
    mockTermsTemplate({
      name,
    }),
  )
}

const baseMocks = [
  {
    request: {
      query: TermsTemplatesDocument,
    },
    result: {
      data: {
        termsTemplates: createRandomTermsTemplates(),
      },
    },
  },
]

const meta = {
  title: "Pages/TermsTemplates",
  component: TermPage,
  parameters: {
    layout: "fullscreen",
    nextjs: {
      appDirectory: true,
    },
  },
} satisfies Meta<typeof TermPage>

export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = {
  decorators: [
    (Story) => (
      <MockedProvider mocks={baseMocks} addTypename={false}>
        <Story />
      </MockedProvider>
    ),
  ],
}
