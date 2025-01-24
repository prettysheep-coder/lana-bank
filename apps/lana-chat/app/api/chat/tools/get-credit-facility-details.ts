import { z } from "zod";
import { tool } from "ai";
import { gql } from "@apollo/client";
import {
  getCreditFacilityDetails,
  GetCreditFacilityDetailsQueryVariables,
} from "@/lib/graphql/generated";

gql`
  query GetCreditFacilityDetails($id: UUID!) {
    creditFacility(id: $id) {
      creditFacilityId
      activatedAt
      expiresAt
      createdAt
      collateralizationState
      facilityAmount
      collateral
      creditFacilityTerms {
        annualRate
        accrualInterval
        incurrenceInterval
        oneTimeFeeRate
        liquidationCvl
        marginCallCvl
        initialCvl
        duration {
          period
          units
        }
      }
      status
      currentCvl {
        total
        disbursed
      }
      transactions {
        ... on CreditFacilityIncrementalPayment {
          cents
          recordedAt
          txId
        }
        ... on CreditFacilityCollateralUpdated {
          satoshis
          recordedAt
          action
          txId
        }
        ... on CreditFacilityOrigination {
          cents
          recordedAt
          txId
        }
        ... on CreditFacilityCollateralizationUpdated {
          state
          collateral
          outstandingInterest
          outstandingDisbursal
          recordedAt
          price
        }
        ... on CreditFacilityDisbursalExecuted {
          cents
          recordedAt
          txId
        }
        ... on CreditFacilityInterestAccrued {
          cents
          recordedAt
          txId
          days
        }
      }
      disbursals {
        disbursalId
        amount
        createdAt
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
          dueOutstanding {
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
          dueOutstanding {
            usdBalance
          }
        }
        outstanding {
          usdBalance
        }
        dueOutstanding {
          usdBalance
        }
        collateral {
          btcBalance
        }
      }
      customer {
        email
      }
    }
  }
`;

export const getCreditFacilityDetailsTool = tool({
  type: "function",
  description: `Retrieve comprehensive details for a single credit facility. USE ONLY: when complete facility details are asked.`,
  parameters: z.object({
    id: z
      .string()
      .uuid()
      .describe(
        "UUID of the credit facility to fetch detailed information for"
      ),
  }),
  execute: async ({ id }) => {
    return getCreditFacilityDetails({ id });
  },
});
