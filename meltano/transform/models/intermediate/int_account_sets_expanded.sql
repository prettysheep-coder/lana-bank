WITH RECURSIVE account_set_members AS (

	SELECT DISTINCT
		account_set_id,
		member_id,
		member_type,

	FROM {{ ref('int_account_set_members') }}

), account_set_members_expanded AS (

	SELECT account_set_id, member_id, member_type,
		[account_set_id] AS set_hierarchy,
	FROM account_set_members

	UNION ALL

	SELECT l.account_set_id, r.member_id, r.member_type,
		ARRAY_CONCAT(l.set_hierarchy, [r.account_set_id]) AS set_hierarchy,
	FROM account_set_members_expanded l
		LEFT JOIN	account_set_members r
			ON l.member_id = r.account_set_id
	WHERE l.member_type = 'AccountSet'

)

SELECT account_set_id, member_id, member_type,
	ANY_VALUE(set_hierarchy HAVING MAX ARRAY_LENGTH(set_hierarchy)) AS set_hierarchy,

FROM account_set_members_expanded

WHERE member_id IS NOT NULL

GROUP BY account_set_id, member_id, member_type
