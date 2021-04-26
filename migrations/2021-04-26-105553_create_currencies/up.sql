CREATE TABLE "public"."currencies" (
    "id" SERIAL,
    "guild_id" bigint NOT NULL,
    "name" text NOT NULL,
    "roles" bigint[] NOT NULL,
    "gain_rate" int NOT NULL,
    PRIMARY KEY ("id")
);

CREATE INDEX "currency_table_guild_id_key" ON "public"."currencies"("guild_id");
