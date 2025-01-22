import { openai } from "@ai-sdk/openai";
import { streamText, tool } from "ai";
import { z } from "zod";
import {
  getCustomerCreditFacility,
  getCustomerDetails,
} from "./get-customer-details";

import {
  CreditFacilitiesFilterBy,
  CreditFacilitiesSortBy,
  CollateralizationState,
  CreditFacilityStatus,
  SortDirection,
} from "@/lib/graphql/generated";
import {
  getCreditFacilities,
  getCreditFacilityById,
} from "./get-facility-details";

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

const CreditFacilitiesFilterSchema = z.object({
  collateralizationState: CollateralizationStateSchema.optional(),
  field: CreditFacilitiesFilterBySchema,
  status: CreditFacilityStatusSchema.optional(),
});

const CreditFacilitiesSortSchema = z.object({
  by: CreditFacilitiesSortBySchema.optional(),
  direction: SortDirectionSchema.optional(),
});

export async function POST(req: Request) {
  const { messages } = await req.json();

  const result = streamText({
    model: openai("gpt-4o-mini"),
    messages,
    maxSteps: 5,
    tools: {
      getCustomerDetails: tool({
        type: "function",
        description:
          "Retrieve general information about a customer by their email address. This includes account balances, deposit history, withdrawal history, KYC document statuses, and basic customer information. **Note**: This does not include any details about the customer's credit facility.",
        parameters: z.object({
          email: z
            .string()
            .describe(
              "The email address of the customer whose general details (excluding credit facility) are being requested."
            ),
        }),
        execute: async ({ email }) => {
          return getCustomerDetails({ email });
        },
      }),
      getCustomerCreditFacilities: tool({
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
      }),
      getCreditFacilities: tool({
        type: "function",
        description:
          "Retrieve list of credit facilities with filtering and sorting. Returns facility details including customer info, balances, and CVL data. Limited to 5 facilities per request.",
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
          console.log({ first, after, sort, filter });
          return getCreditFacilities({
            first,
            after,
            sort,
            filter,
          });
        },
      }),
      getCreditFacilityDetails: tool({
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
          console.log(id);
          return getCreditFacilityById({ id });
        },
      }),
    },
  });

  return result.toDataStreamResponse();
}
