export const databaseConfig = {
  host: process.env.DB_HOST || "localhost",
  port: parseInt(process.env.DB_PORT || "", 10) || 5435,
  user: process.env.DB_USER || "user",
  password: process.env.DB_PWD || "password",
  database: process.env.DB_DB || "pg",
  poolMin: parseInt(process.env.DB_POOL_MIN || "", 10) || 1,
  poolMax: parseInt(process.env.DB_POOL_MAX || "", 10) || 5,
  debug: process.env.DB_DEBUG === "true",
}
