import { z } from "zod";
import { tool } from "ai";
import { gql } from "@apollo/client";
import { getClient } from "../client";
import {
  GetCustomerByEmailDocument,
  GetCustomerByEmailQuery,
  GetCustomerByEmailQueryVariables,
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

export const getCustomerDetailsTool = tool({
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
});
