import type { Knex } from "knex";


export async function up(knex: Knex): Promise<void> {
  return knex.schema
    .createTable('verification_token', function(table) {
      table.text('identifier').notNullable();
      table.timestamp('expires').notNullable();
      table.text('token').notNullable();
      table.primary(['identifier', 'token']);
    })
    .createTable('accounts', function(table) {
      table.increments('id');
      table.integer('userId').notNullable();
      table.string('type', 255).notNullable();
      table.string('provider', 255).notNullable();
      table.string('providerAccountId', 255).notNullable();
      table.text('refresh_token');
      table.text('access_token');
      table.bigint('expires_at');
      table.text('id_token');
      table.text('scope');
      table.text('session_state');
      table.text('token_type');
    })
    .createTable('sessions', function(table) {
      table.increments('id');
      table.integer('userId').notNullable();
      table.timestamp('expires').notNullable();
      table.string('sessionToken', 255).notNullable();
    })
    .createTable('users', function(table) {
      table.increments('id');
      table.string('name', 255);
      table.string('email', 255);
      table.timestamp('emailVerified');
      table.text('image');
    });
}


export async function down(knex: Knex): Promise<void> {
  return knex.schema
    .dropTable('verification_token')
    .dropTable('accounts')
    .dropTable('sessions')
    .dropTable('users');
}

