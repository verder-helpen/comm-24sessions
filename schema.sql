DROP TABLE IF EXISTS "session";

CREATE TABLE "session" (
    "id" SERIAL NOT NULL,
    "session_id" text NOT NULL,
    "room_id"  text NOT NULL,
    "domain" text NOT NULL,
    "redirect_url" text NOT NULL,
    "purpose" text NOT NULL,
    "name" text NOT NULL,
    "instance" text NOT NULL,
    "attr_id" text NOT NULL,
    "auth_result" text,
    "last_activity" timestamp NOT NULL,
    PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX ON "session" ("attr_id");
CREATE UNIQUE INDEX ON "session" ("session_id");
