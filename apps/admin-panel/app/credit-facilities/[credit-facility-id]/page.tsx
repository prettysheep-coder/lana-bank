"use client"

import { gql } from "@apollo/client"

import { CreditFacilityOverview } from "./overview"

import { useGetCreditFacilityOverviewQuery } from "@/lib/graphql/generated"

gql`
  query GetCreditFacilityOverview($id: UUID!) {
    creditFacility(id: $id) {
      id
      creditFacilityId
      status
      facilityAmount
      collateral
      expiresAt
      currentCvl {
        total
        disbursed
      }
      collateralToMatchInitialCvl @client
      disbursals {
        status
      }
      balance {
        facilityRemaining {
          usdBalance
        }
        disbursed {
          total {
            usdBalance
          }
          outstanding {
            usdBalance
          }
        }
        interest {
          total {
            usdBalance
          }
          outstanding {
            usdBalance
          }
        }
        outstanding {
          usdBalance
        }
        collateral {
          btcBalance
        }
      }
      creditFacilityTerms {
        marginCallCvl
        liquidationCvl
        initialCvl
      }
      approvalProcess {
        approvalProcessId
        approvalProcessType
        deniedReason
        createdAt
        subjectCanSubmitDecision
        status
        rules {
          ... on CommitteeThreshold {
            threshold
            committee {
              name
              currentMembers {
                email
                roles
              }
            }
          }
          ... on SystemApproval {
            autoApprove
          }
        }
        voters {
          stillEligible
          didVote
          didApprove
          didDeny
          user {
            userId
            email
            roles
          }
        }
      }
    }
  }
`

export default function CreditFacilityPage({
  params,
}: {
  params: { "credit-facility-id": string }
}) {
  const { data } = useGetCreditFacilityOverviewQuery({
    variables: { id: params["credit-facility-id"] },
  })

  if (!data?.creditFacility) return null

  return <CreditFacilityOverview creditFacility={data.creditFacility} />
}
