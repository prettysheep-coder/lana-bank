config {
	type: "operations",
	hasOutput: true,
	description: "Asserts that the value is a valid code in the static data table."
}

-- CREATE OR REPLACE PROCEDURE ${self()} (IN assert_value STRING, IN dim_table_name STRING, OUT result BOOLEAN)
CREATE OR REPLACE PROCEDURE ${self()} (IN assert_value STRING, IN dim_table_name STRING)
BEGIN
    EXECUTE IMMEDIATE format("""
      SELECT COALESCE('%s', '~') NOT IN (SELECT `code` FROM %s) AS error
    """, assert_value, dim_table_name)
    -- INTO result;
    ;
END;
