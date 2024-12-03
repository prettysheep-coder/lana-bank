"use client"

import { gql } from "@apollo/client"

import { CreditFacilityDisbursals } from "./list"

import { useGetCreditFacilityDisbursalsQuery } from "@/lib/graphql/generated"

gql`
  query GetCreditFacilityDisbursals($id: UUID!) {
    creditFacility(id: $id) {
      id
      creditFacilityId
      disbursals {
        id
        disbursalId
        index
        amount
        status
        createdAt
        approvalProcess {
          approvalProcessId
          deniedReason
          approvalProcessType
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
  }
`
export default function CreditFacilityDisbursalsPage({
  params,
}: {
  params: { "credit-facility-id": string }
}) {
  const { data } = useGetCreditFacilityDisbursalsQuery({
    variables: { id: params["credit-facility-id"] },
  })

  if (!data?.creditFacility) return null

  return <CreditFacilityDisbursals creditFacility={data.creditFacility} />
}
