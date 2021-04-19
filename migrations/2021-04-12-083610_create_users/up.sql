CREATE TABLE "public"."user" (
    "id" SERIAL,
    "guild_id" BIGINT,
    "discord_id" BIGINT,
    "name" text,
    "balance" BIGINT,
    "last_active" timestamp with time zone,
    PRIMARY KEY ("id")
);
