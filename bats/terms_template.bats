load "helpers"

setup_file() {
  start_server
}

teardown_file() {
  stop_server
}

@test "terms-template: create" {
  template_name="Test Template $(date +%s)"

  variables=$(
    jq -n \
    --arg name "$template_name" \
    '{
      input: {
        name: $name,
        annualRate: 5.5,
        interval: "END_OF_MONTH",
        duration: {
          period: "MONTHS",
          units: 12
        },
        liquidationCvl: 80,
        marginCallCvl: 90,
        initialCvl: 100
      }
    }'
  )

  exec_admin_graphql 'terms-template-create' "$variables"

  terms_template_id=$(graphql_output .data.termsTemplateCreate.termsTemplate.termsId)
  [[ "$terms_template_id" != "null" ]] || exit 1
  annual_rate=$(graphql_output .data.termsTemplateCreate.termsTemplate.values.annualRate)
  [[ "$annual_rate" == "5.5" ]] || exit 1
  interval=$(graphql_output .data.termsTemplateCreate.termsTemplate.values.interval)
  [[ "$interval" == "END_OF_MONTH" ]] || exit 1
  duration_period=$(graphql_output .data.termsTemplateCreate.termsTemplate.values.duration.period)
  [[ "$duration_period" == "MONTHS" ]] || exit 1
  duration_units=$(graphql_output .data.termsTemplateCreate.termsTemplate.values.duration.units)
  [[ "$duration_units" == "12" ]] || exit 1
  liquidation_cvl=$(graphql_output .data.termsTemplateCreate.termsTemplate.values.liquidationCvl)
  [[ "$liquidation_cvl" == "80" ]] || exit 1
  margin_call_cvl=$(graphql_output .data.termsTemplateCreate.termsTemplate.values.marginCallCvl)
  [[ "$margin_call_cvl" == "90" ]] || exit 1
  initial_cvl=$(graphql_output .data.termsTemplateCreate.termsTemplate.values.initialCvl)
  [[ "$initial_cvl" == "100" ]] || exit 1
}

@test "terms-template: update" {
  template_name="Test Template for Update $(date +%s)"

  create_variables=$(
    jq -n \
    --arg name "$template_name" \
    '{
      input: {
        name: $name,
        annualRate: 5.5,
        interval: "END_OF_MONTH",
        duration: {
          period: "MONTHS",
          units: 12
        },
        liquidationCvl: 80,
        marginCallCvl: 90,
        initialCvl: 100
      }
    }'
  )


  exec_admin_graphql 'terms-template-create' "$create_variables"

  create_response=$(graphql_output)

  terms_template_id=$(echo "$create_response" | jq -r '.data.termsTemplateCreate.termsTemplate.termsId')
  [[ "$terms_template_id" != "null" ]] || exit 1

  update_variables=$(
    jq -n \
    --arg id "$terms_template_id" \
    '{
      input: {
        id: $id,
        annualRate: 6.5,
        interval: "END_OF_MONTH",
        duration: {
          period: "MONTHS",
          units: 24
        },
        liquidationCvl: 75,
        marginCallCvl: 85,
        initialCvl: 95
      }
    }'
  )


  exec_admin_graphql 'terms-template-update' "$update_variables"

  update_response=$(graphql_output)
  updated_id=$(echo "$update_response" | jq -r '.data.termsTemplateUpdate.termsTemplate.termsId')
  [[ "$updated_id" == "$terms_template_id" ]] || exit 1
  annual_rate=$(echo "$update_response" | jq -r '.data.termsTemplateUpdate.termsTemplate.values.annualRate')
  [[ "$annual_rate" == "6.5" ]] || exit 1
  interval=$(echo "$update_response" | jq -r '.data.termsTemplateUpdate.termsTemplate.values.interval')
  [[ "$interval" == "END_OF_MONTH" ]] || exit 1
  duration_period=$(echo "$update_response" | jq -r '.data.termsTemplateUpdate.termsTemplate.values.duration.period')
  [[ "$duration_period" == "MONTHS" ]] || exit 1
  duration_units=$(echo "$update_response" | jq -r '.data.termsTemplateUpdate.termsTemplate.values.duration.units')
  [[ "$duration_units" == "24" ]] || exit 1
  liquidation_cvl=$(echo "$update_response" | jq -r '.data.termsTemplateUpdate.termsTemplate.values.liquidationCvl')
  [[ "$liquidation_cvl" == "75" ]] || exit 1
  margin_call_cvl=$(echo "$update_response" | jq -r '.data.termsTemplateUpdate.termsTemplate.values.marginCallCvl')
  [[ "$margin_call_cvl" == "85" ]] || exit 1
  initial_cvl=$(echo "$update_response" | jq -r '.data.termsTemplateUpdate.termsTemplate.values.initialCvl')
  [[ "$initial_cvl" == "95" ]] || exit 1
}