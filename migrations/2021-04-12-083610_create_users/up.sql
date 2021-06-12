CREATE TABLE "public"."users" (
    "id" SERIAL,
    "discord_id" text NOT NULL,
    "last_active" timestamp with time zone,
    PRIMARY KEY ("id")
);
