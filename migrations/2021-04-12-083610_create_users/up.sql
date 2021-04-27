CREATE TABLE "public"."users" (
    "id" SERIAL,
    "discord_id" BIGINT NOT NULL,
    "last_active" timestamp with time zone,
    PRIMARY KEY ("id")
);
