ALTER TABLE "public"."users"
  ADD COLUMN "steam_id" text,
  ADD UNIQUE ("steam_id");
