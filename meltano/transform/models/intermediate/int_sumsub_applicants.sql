select
    customer_id,
    json_value(content, "$.id") as applicant_id,
    timestamp(json_value(content, "$.createdAt")) as created_at,
    json_value(content, "$.info.firstName") as first_name,
    json_value(content, "$.info.lastName") as last_name,
    date(json_value(content, "$.info.dob")) as date_of_birth,
    json_value(content, "$.info.gender") as gender,
    json_value(content, "$.info.country") as iso_alpha_3_code,
    json_value(content, "$.info.nationality") as nationality_iso_alpha_3_code,
    array(
        select as struct
            json_value(doc, "$.country") as iso_alpha_3_code,
            json_value(doc, "$.idDocType") as document_type,
            json_value(doc, "$.number") as number
        from unnest(json_query_array(content, "$.info.idDocs")) as doc
    ) as id_documents,

    array(
        select as struct
            json_value(questions, "$.sections.personalInformation.items.occupation.value")
                as occupation_code,
            json_value(questions, "$.sections.personalInformation.items.economicActivity.value")
                as economic_activity_code,
            json_value(questions, "$.sections.personalInformation.items.countryOfResidence.value")
                as country_of_residence_iso_alpha_3_code,
            json_value(
                questions, "$.sections.personalInformation.items.taxIdentificationNum.value"
            ) as tax_id_number
        from unnest(json_query_array(content, "$.questionnaires")) as questions
    ) as questionnaires

from {{ ref('stg_sumsub_applicants') }}
