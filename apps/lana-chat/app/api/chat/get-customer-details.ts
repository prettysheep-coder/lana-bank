import { gql } from "@apollo/client";
import { getClient } from "./client";
import {
  GetCustomerByEmailQuery,
  GetCustomerByEmailQueryVariables,
} from "@/lib/graphql/generated";

const GET_CUSTOMER_BY_EMAIL = gql`
  query GetCustomerByEmail($email: String!) {
    customerByEmail(email: $email) {
      id
      customerId
      depositAccount {
        balance {
          settled
          pending
        }
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
      query: GET_CUSTOMER_BY_EMAIL,
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
