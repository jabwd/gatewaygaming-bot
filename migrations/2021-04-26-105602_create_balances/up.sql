CREATE TABLE "public"."balances" (
    "id" SERIAL,
    "user_id" integer NOT NULL,
    "currency_id" integer NOT NULL,
    "value" bigint NOT NULL,
    PRIMARY KEY ("id"),
    FOREIGN KEY ("currency_id") REFERENCES "public"."currencies"("id") ON DELETE CASCADE,
    FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE CASCADE
);
