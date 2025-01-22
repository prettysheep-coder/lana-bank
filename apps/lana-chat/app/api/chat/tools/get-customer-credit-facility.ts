import { z } from "zod";
import { tool } from "ai";
import { gql } from "@apollo/client";
import { getClient } from "../client";
import {
  GetCustomerCreditFacilityByEmailDocument,
  GetCustomerCreditFacilityByEmailQuery,
  GetCustomerCreditFacilityByEmailQueryVariables,
} from "@/lib/graphql/generated";

gql`
  query GetCustomerCreditFacilityByEmail($email: String!) {
    customerByEmail(email: $email) {
      customerId
      creditFacilities {
        creditFacilityId
        activatedAt
        expiresAt
        createdAt
        collateralizationState
        facilityAmount
        collateral
        canBeCompleted
        status
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
        currentCvl {
          total
          disbursed
        }
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
      }
    }
  }
`;

export const getCustomerCreditFacility = async (
  variables: GetCustomerCreditFacilityByEmailQueryVariables
) => {
  try {
    const response = await getClient().query<
      GetCustomerCreditFacilityByEmailQuery,
      GetCustomerCreditFacilityByEmailQueryVariables
    >({
      query: GetCustomerCreditFacilityByEmailDocument,
      variables,
    });

    if (!response.data?.customerByEmail) {
      return { error: "Customer not found" };
    }

    return response;
  } catch (error) {
    if (error instanceof Error) {
      return { error: error.message };
    }
    return { error: "An unknown error occurred" };
  }
};

export const getCustomerCreditFacilitiesTool = tool({
  type: "function",
  description:
    "Retrieve details about the customer's credit facility, including the facility amount, transactions, balance information, collateral, and associated terms. Use this tool to get credit-specific information about the customer.",
  parameters: z.object({
    email: z
      .string()
      .describe(
        "The email address of the customer whose credit facility details are being requested."
      ),
  }),
  execute: async ({ email }) => {
    return getCustomerCreditFacility({ email });
  },
});
