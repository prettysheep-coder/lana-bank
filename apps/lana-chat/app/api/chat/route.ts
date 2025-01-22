import { openai } from "@ai-sdk/openai";
import { streamText, tool } from "ai";
import { z } from "zod";
import {
  getCustomerCreditFacility,
  getCustomerDetails,
} from "./get-customer-details";

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
          "Retrieve general information about a customer by their email address. This includes account balances, deposit history, withdrawal history, KYC document statuses, and basic customer information. **Note**: This does not include any details about the customer's credit facility (if they have one).",
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
      getCustomerCreditFacility: tool({
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
    },
  });

  return result.toDataStreamResponse();
}
