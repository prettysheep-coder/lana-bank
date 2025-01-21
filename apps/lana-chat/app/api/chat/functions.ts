import { getCustomerDetails } from "./get-customer-details";

export const tools = [
  {
    type: "function" as const,
    function: {
      name: "get_customer_details",
      description: "Get Details for a Customer searching by email",
      parameters: {
        type: "object",
        properties: {
          email: {
            type: "string",
            description: "Email of the Customer",
          },
        },
        required: ["email"],
        additionalProperties: false,
      },
      strict: true,
    },
  },
];

export async function runFunction(name: string, args: any) {
  switch (name) {
    case "get_customer_details":
      return getCustomerDetails(args.email);
    default:
      throw new Error(`Unknown function: ${name}`);
  }
}
