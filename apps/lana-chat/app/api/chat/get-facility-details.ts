import { gql } from "@apollo/client";

import {
  CreditFacilitiesSort,
  CreditFacilitiesFilter,
  CreditFacilitiesQueryVariables,
  CreditFacilitiesQuery,
  CreditFacilitiesDocument,
  GetCreditFacilityDetailsQueryVariables,
  GetCreditFacilityDetailsQuery,
  GetCreditFacilityDetailsDocument,
} from "@/lib/graphql/generated";
import { getClient } from "./client";

gql`
  query CreditFacilities(
    $first: Int!
    $after: String
    $sort: CreditFacilitiesSort
    $filter: CreditFacilitiesFilter
  ) {
    creditFacilities(
      first: $first
      after: $after
      sort: $sort
      filter: $filter
    ) {
      edges {
        cursor
        node {
          id
          creditFacilityId
          collateralizationState
          createdAt
          status
          facilityAmount
          collateral
          currentCvl {
            disbursed
            total
          }
          balance {
            collateral {
              btcBalance
            }
            outstanding {
              usdBalance
            }
          }
          customer {
            customerId
            email
          }
        }
      }
      pageInfo {
        endCursor
        hasNextPage
      }
    }
  }
`;

export const getCreditFacilities = async (
  variables: CreditFacilitiesQueryVariables
) => {
  try {
    const response = await getClient().query<
      CreditFacilitiesQuery,
      CreditFacilitiesQueryVariables
    >({
      query: CreditFacilitiesDocument,
      variables,
    });

    if (!response.data) {
      return { error: "Data not found" };
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

export const getCreditFacilityById = async (
  variables: GetCreditFacilityDetailsQueryVariables
) => {
  try {
    const response = await getClient().query<
      GetCreditFacilityDetailsQuery,
      GetCreditFacilityDetailsQueryVariables
    >({
      query: GetCreditFacilityDetailsDocument,
      variables,
    });

    if (!response.data) {
      return { error: "Data not found" };
    }

    return response;
  } catch (error) {
    if (error instanceof Error) {
      return { error: error.message };
    }
    return { error: "An unknown error occurred" };
  }
};
