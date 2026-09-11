CREATE TABLE IF NOT EXISTS "assembly" (
	"id" INTEGER PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"sku" VARCHAR,
	"msrp" NUMERIC,
	"price" NUMERIC,
	"cost" NUMERIC,
	"quantity" INTEGER,
	"weight" NUMERIC,
	"dimensions" NUMERIC,
	"discounts" INTEGER,
	"image" BLOB,
	"tax_class" INTEGER,
	"vendor" INTEGER,
	FOREIGN KEY ("id") REFERENCES "service_order"("item")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "service"("item")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "sale"("items")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "junc_tag_assembly"("assembly")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "junc_item_assembly"("assembly")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "junc_service_assembly"("assembly")
	ON UPDATE NO ACTION ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS "appointment" (
	"id" INTEGER PRIMARY KEY,
	"date" DATETIME NOT NULL,
	"name" VARCHAR NOT NULL,
	"approved" BOOLEAN NOT NULL,
	"customer" INTEGER NOT NULL,
	"service" INTEGER NOT NULL,
	FOREIGN KEY ("id") REFERENCES "junc_business_appointment"("appointment")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "junc_service_appointment"("appointment")
	ON UPDATE NO ACTION ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS "box" (
	"id" INTEGER PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"sku" VARCHAR,
	"msrp" NUMERIC,
	"price" NUMERIC,
	"cost" NUMERIC,
	"quantity" INTEGER,
	"weight" NUMERIC,
	"dimensions" NUMERIC,
	"discounts" INTEGER,
	"image" BLOB,
	"tax_class" INTEGER,
	"vendor" INTEGER,
	"contained_quantity" INTEGER,
	"item" INTEGER NOT NULL,
	FOREIGN KEY ("id") REFERENCES "service_order"("item")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "junc_tag_box"("box")
	ON UPDATE NO ACTION ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS "business" (
	"id" INTEGER PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"api_key" VARCHAR NOT NULL,
	FOREIGN KEY ("id") REFERENCES "junc_business_appointment"("business")
	ON UPDATE NO ACTION ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS "category" (
	"id" INTEGER PRIMARY KEY,
	"id" INTEGER NOT NULL UNIQUE,
	"name" VARCHAR NOT NULL,
	FOREIGN KEY ("id") REFERENCES "service"("category")
	ON UPDATE NO ACTION ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS "customer" (
	"id" INTEGER PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"date_created" DATE NOT NULL,
	"email" VARCHAR,
	"phone_number" VARCHAR,
	FOREIGN KEY ("id") REFERENCES "service_order"("customer")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "appointment"("customer")
	ON UPDATE NO ACTION ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS "discount" (
	"id" INTEGER PRIMARY KEY,
	"type" VARCHAR,
	"amount" INTEGER,
	FOREIGN KEY ("id") REFERENCES "item"("discounts")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "box"("discounts")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "assembly"("discounts")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "sale"("discount")
	ON UPDATE NO ACTION ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS "employee" (
	"id" INTEGER PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"date_created" DATE NOT NULL,
	"identifier" VARCHAR NOT NULL UNIQUE,
	"email" VARCHAR,
	"phone_number" VARCHAR,
	"role" VARCHAR,
	FOREIGN KEY ("id") REFERENCES "service_order"("employee")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "service"("employee")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "sale"("employee")
	ON UPDATE NO ACTION ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS "item" (
	"id" INTEGER PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"sku" VARCHAR,
	"msrp" NUMERIC,
	"price" NUMERIC,
	"cost" NUMERIC,
	"quantity" INTEGER,
	"weight" NUMERIC,
	"dimensions" NUMERIC,
	"discounts" INTEGER,
	"image" BLOB,
	"tax_class" INTEGER,
	"vendor" INTEGER,
	FOREIGN KEY ("id") REFERENCES "box"("item")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "service_order"("item")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "service"("item")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "sale"("items")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "junc_tag_item"("item")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "junc_item_assembly"("item")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "junc_service_item"("item")
	ON UPDATE NO ACTION ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS "labour" (
	"id" INTEGER PRIMARY KEY,
	"id" INTEGER NOT NULL UNIQUE,
	"name" VARCHAR NOT NULL,
	"sku" VARCHAR,
	"price" NUMERIC,
	"cost" NUMERIC,
	"discounts" INTEGER,
	"tax_class" INTEGER,
	"vendor" INTEGER,
	"duration" INTEGER NOT NULL,
	FOREIGN KEY ("id") REFERENCES "service_order"("item")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "service"("item")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "sale"("items")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "junc_tag_labour"("labour")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "junc_service_labour"("labour")
	ON UPDATE NO ACTION ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS "purchase_order" (
	"id" INTEGER PRIMARY KEY,
	"id" INTEGER NOT NULL UNIQUE,
	"items" INTEGER NOT NULL,
	"vendor" INTEGER,
	"order_id" VARCHAR NOT NULL,
	"name(alias)" VARCHAR,
	"order_date" DATE,
	"arrival_date" DATE,
	"status" VARCHAR NOT NULL,
	FOREIGN KEY ("items") REFERENCES "item"("id")
	ON UPDATE NO ACTION ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS "sale" (
	"id" INTEGER PRIMARY KEY,
	"id" INTEGER NOT NULL UNIQUE,
	"date" DATE NOT NULL,
	"customer" INTEGER NOT NULL,
	"items" INTEGER NOT NULL,
	"services" INTEGER NOT NULL,
	"discount" INTEGER NOT NULL,
	"employee" INTEGER NOT NULL,
	FOREIGN KEY ("customer") REFERENCES "customer"("id")
	ON UPDATE NO ACTION ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS "service" (
	"id" INTEGER PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"item" INTEGER,
	"category" INTEGER NOT NULL,
	"employee" INTEGER,
	FOREIGN KEY ("id") REFERENCES "sale"("services")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "junc_service_appointment"("service")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "junc_service_labour"("service")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "junc_service_item"("service")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "junc_service_assembly"("service")
	ON UPDATE NO ACTION ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS "service_order" (
	"id" INTEGER PRIMARY KEY,
	"id" INTEGER NOT NULL UNIQUE,
	"date_created" DATE NOT NULL,
	"date_due" DATE,
	"employee" INTEGER NOT NULL,
	"customer" INTEGER NOT NULL,
	"receipt_notes" VARCHAR,
	"internal_notes" VARCHAR,
	"item" INTEGER,
);

CREATE TABLE IF NOT EXISTS "tag" (
	"id" INTEGER PRIMARY KEY,
	"name" VARCHAR NOT NULL UNIQUE,
	FOREIGN KEY ("id") REFERENCES "junc_tag_labour"("tag")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "junc_tag_item"("tag")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "junc_tag_assembly"("tag")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "junc_tag_box"("tag")
	ON UPDATE NO ACTION ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS "vendor" (
	"id" INTEGER PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	FOREIGN KEY ("id") REFERENCES "purchase_order"("vendor")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "item"("vendor")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "assembly"("vendor")
	ON UPDATE NO ACTION ON DELETE NO ACTION,
	FOREIGN KEY ("id") REFERENCES "box"("vendor")
	ON UPDATE NO ACTION ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS "junc_business_appointment" (
	"appointment" INTEGER NOT NULL,
	"business" INTEGER NOT NULL,
	PRIMARY KEY("appointment", "business")
);

CREATE TABLE IF NOT EXISTS "junc_service_appointment" (
	"service" INTEGER NOT NULL,
	"appointment" INTEGER NOT NULL,
	PRIMARY KEY("service", "appointment")
);

CREATE TABLE IF NOT EXISTS "junc_tag_labour" (
	"tag" INTEGER NOT NULL,
	"labour" INTEGER NOT NULL,
	PRIMARY KEY("tag", "labour")
);

CREATE TABLE IF NOT EXISTS "junc_tag_item" (
	"tag" INTEGER NOT NULL,
	"item" INTEGER NOT NULL,
	PRIMARY KEY("tag", "item")
);

CREATE TABLE IF NOT EXISTS "junc_tag_assembly" (
	"tag" INTEGER NOT NULL,
	"assembly" INTEGER NOT NULL,
	PRIMARY KEY("tag", "assembly")
);

CREATE TABLE IF NOT EXISTS "junc_item_assembly" (
	"item" INTEGER NOT NULL,
	"assembly" INTEGER NOT NULL,
	PRIMARY KEY("item", "assembly")
);

CREATE TABLE IF NOT EXISTS "junc_tag_box" (
	"tag" INTEGER NOT NULL,
	"box" INTEGER NOT NULL,
	PRIMARY KEY("tag", "box")
);

CREATE TABLE IF NOT EXISTS "junc_service_labour" (
	"labour" INTEGER NOT NULL,
	"service" INTEGER NOT NULL,
	PRIMARY KEY("labour", "service")
);

CREATE TABLE IF NOT EXISTS "junc_service_assembly" (
	"assembly" INTEGER NOT NULL,
	"service" INTEGER NOT NULL,
	PRIMARY KEY("assembly", "service")
);

CREATE TABLE IF NOT EXISTS "junc_service_item" (
	"item" INTEGER NOT NULL,
	"service" INTEGER NOT NULL,
	PRIMARY KEY("item", "service")
);
