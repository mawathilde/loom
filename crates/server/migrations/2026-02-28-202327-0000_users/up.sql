-- Your SQL goes here
CREATE TABLE `users`
(
    `uuid`          TEXT      NOT NULL PRIMARY KEY,
    `created_at`    TIMESTAMP NOT NULL,
    `updated_at`    TIMESTAMP NOT NULL,
    `email`         TEXT      NOT NULL,
    `auth_key_hash` BINARY    NOT NULL,
    `salt`          BINARY    NOT NULL
);

CREATE UNIQUE INDEX `users_email_idx` ON `users` (`email`);

