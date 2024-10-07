import CreditFacilityDetailsCard from "./details"

import { PageHeading } from "@/components/page-heading"

function withdrawalDetails({
  params,
}: {
  params: {
    "credit-facility-id": string
  }
}) {
  const { "credit-facility-id": creditFacilityId } = params

  return (
    <main className="max-w-7xl m-auto">
      <PageHeading>Credit Facility Details</PageHeading>
      <CreditFacilityDetailsCard creditFacilityId={creditFacilityId} />
    </main>
  )
}

export default withdrawalDetails
