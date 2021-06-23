CREATE TABLE "public"."garage" (
    "id" SERIAL,
    "user_id" integer NOT NULL,
    
    PRIMARY KEY ("id"),
    FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE CASCADE
);
