import { gql } from "@apollo/client";
import { getClient } from "./client";
import {
  GetCustomerByEmailDocument,
  GetCustomerByEmailQuery,
  GetCustomerByEmailQueryVariables,
  GetCustomerCreditFacilityByEmailDocument,
  GetCustomerCreditFacilityByEmailQuery,
  GetCustomerCreditFacilityByEmailQueryVariables,
} from "@/lib/graphql/generated";

gql`
  query GetCustomerByEmail($email: String!) {
    customerByEmail(email: $email) {
      customerId
      status
      level
      createdAt
      email
      telegramId
      applicantId
      depositAccount {
        depositAccountId
        balance {
          pending
          settled
        }
        withdrawals {
          amount
          createdAt
          reference
          status
          withdrawalId
        }
        deposits {
          amount
          createdAt
          reference
          depositId
        }
      }
      documents {
        documentId
        status
        filename
      }
    }
  }
`;

export const getCustomerDetails = async (
  variables: GetCustomerByEmailQueryVariables
) => {
  try {
    const response = await getClient().query<
      GetCustomerByEmailQuery,
      GetCustomerByEmailQueryVariables
    >({
      query: GetCustomerByEmailDocument,
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
