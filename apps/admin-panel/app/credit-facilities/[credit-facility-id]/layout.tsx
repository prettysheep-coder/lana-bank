"use client"

import { gql } from "@apollo/client"

import CreditFacilityDetailsCard from "./details"

import { BreadcrumbLink, BreadCrumbWrapper } from "@/components/breadcrumb-wrapper"
import { DetailsPageSkeleton } from "@/components/details-page-skeleton"
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/ui/tab"
import { useTabNavigation } from "@/hooks/use-tab-navigation"

import { useGetCreditFacilityBasicDetailsQuery } from "@/lib/graphql/generated"

gql`
  query GetCreditFacilityBasicDetails($id: UUID!) {
    creditFacility(id: $id) {
      id
      creditFacilityId
      status
      facilityAmount
      collateralizationState
      customer {
        customerId
        email
      }
      approvalProcess {
        id
        deniedReason
        status
        subjectCanSubmitDecision
        approvalProcessId
        approvalProcessType
        createdAt
      }
      subjectCanUpdateCollateral
      subjectCanInitiateDisbursal
      subjectCanRecordPayment
      subjectCanComplete
    }
  }
`

const TABS = [
  { url: "/", tabLabel: "Overview" },
  { url: "/terms", tabLabel: "Terms" },
  { url: "/transactions", tabLabel: "Transactions" },
  { url: "/disbursals", tabLabel: "Disbursals" },
]

export default function CreditFacilityLayout({
  children,
  params,
}: {
  children: React.ReactNode
  params: { "credit-facility-id": string }
}) {
  const { "credit-facility-id": creditFacilityId } = params
  const { currentTab, handleTabChange } = useTabNavigation(TABS, creditFacilityId)

  const { data, loading, error, refetch } = useGetCreditFacilityBasicDetailsQuery({
    variables: { id: creditFacilityId },
  })

  if (loading) return <DetailsPageSkeleton detailItems={4} tabs={4} />
  if (error) return <div className="text-destructive">{error.message}</div>
  if (!data?.creditFacility) return <div>Not found</div>

  const currentTabData = TABS.find((tab) => tab.url === currentTab)

  const links: BreadcrumbLink[] = [
    { title: "Dashboard", href: "/dashboard" },
    { title: "Credit Facilities", href: "/credit-facilities" },
    {
      title: `${data.creditFacility.customer.email} - ${data.creditFacility.creditFacilityId}`,
      href: `/credit-facilities/${creditFacilityId}`,
    },
    ...(currentTabData?.url === "/"
      ? []
      : [{ title: currentTabData?.tabLabel ?? "", isCurrentPage: true as const }]),
  ]

  return (
    <main className="max-w-7xl m-auto">
      <BreadCrumbWrapper links={links} />
      <CreditFacilityDetailsCard
        creditFacilityId={creditFacilityId}
        creditFacilityDetails={data.creditFacility}
        refetch={refetch}
      />

      <Tabs value={currentTab} onValueChange={handleTabChange} className="mt-2">
        <TabsList>
          {TABS.map((tab) => (
            <TabsTrigger key={tab.url} value={tab.url}>
              {tab.tabLabel}
            </TabsTrigger>
          ))}
        </TabsList>
        {TABS.map((tab) => (
          <TabsContent key={tab.url} value={tab.url}>
            {children}
          </TabsContent>
        ))}
      </Tabs>
    </main>
  )
}
