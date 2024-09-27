config {
	type: "operations",
	hasOutput: true,
	description: "Asserts that the value is a valid code in the static data table."
}

CREATE OR REPLACE FUNCTION ${self()} (assert_value STRING, dim_table_name STRING)
RETURNS BOOLEAN
AS (
  (
    dataform_sv.udp_nbp4_17_assert_in_string_dim_table(assert_value, dim_table_name, result);
    -- ${ref('udp_nbp4_17_assert_in_string_dim_table')}
  )
);
