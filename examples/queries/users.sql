-- name: GetUserByID :one
SELECT *
FROM users
WHERE id = :id;

-- name: DeleteUser :exec
DELETE
FROM users
WHERE id = :id;