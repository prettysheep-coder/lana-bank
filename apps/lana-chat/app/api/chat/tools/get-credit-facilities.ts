import { z } from "zod";
import { tool } from "ai";
import { gql } from "@apollo/client";
import { getClient } from "../client";
import {
  CreditFacilitiesDocument,
  CreditFacilitiesQuery,
  CreditFacilitiesQueryVariables,
  CreditFacilitiesFilterBy,
  CreditFacilitiesSortBy,
  CollateralizationState,
  CreditFacilityStatus,
  SortDirection,
} from "@/lib/graphql/generated";

const CollateralizationStateSchema = z
  .nativeEnum(CollateralizationState)
  .describe(`Collateralization states`);

const CreditFacilityStatusSchema = z
  .nativeEnum(CreditFacilityStatus)
  .describe(`Facility statuses`);

const CreditFacilitiesFilterBySchema = z
  .nativeEnum(CreditFacilitiesFilterBy)
  .describe(`Filter by fields`);

const CreditFacilitiesSortBySchema = z
  .nativeEnum(CreditFacilitiesSortBy)
  .describe(`Sort by fields`);

const SortDirectionSchema = z
  .nativeEnum(SortDirection)
  .describe(`Sort directions`);

const CreditFacilitiesFilterSchema = z
  .object({
    collateralizationState: CollateralizationStateSchema.optional().describe(
      "Filter facilities by collateralization state"
    ),
    field: CreditFacilitiesFilterBySchema.describe(
      "Required field to filter facilities by"
    ),
    status: CreditFacilityStatusSchema.optional().describe(
      "Filter facilities by status"
    ),
  })
  .nullish()
  .describe("Optional filters for credit facilities query");

const CreditFacilitiesSortSchema = z.object({
  by: CreditFacilitiesSortBySchema.optional().describe(
    "Field to sort facilities by"
  ),
  direction: SortDirectionSchema.optional().describe(
    "Sort direction (ascending/descending)"
  ),
});

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

export const getCreditFacilitiesTool = tool({
  type: "function",
  description:
    "Retrieve list of credit facilities with filtering and sorting. When using filter with status or collateralizationState, you must also specify field as 'STATUS' or 'COLLATERALIZATION_STATE' respectively. Limited to 5 facilities per request.",
  parameters: z.object({
    first: z
      .number()
      .min(1)
      .max(5)
      .describe("Number of facilities to fetch (5 max: cannot exceed 5)"),
    after: z
      .string()
      .optional()
      .describe("Pagination cursor. Null/undefined for first page"),
    sort: CreditFacilitiesSortSchema.optional(),
    filter: CreditFacilitiesFilterSchema.optional(),
  }),
  execute: async ({ first, after, sort, filter }) => {
    return getCreditFacilities({
      first,
      after,
      sort,
      filter,
    });
  },
});
