---------------------------------------------------
---                  category                   ---
---------------------------------------------------
CREATE TABLE category ( 
	id                   INT NOT NULL  PRIMARY KEY  ,
	name                 TEXT NOT NULL    ,
	description          TEXT     
 );

---------------------------------------------------
---                  customer                   ---
---------------------------------------------------
CREATE TABLE customer ( 
	id                   INT NOT NULL  PRIMARY KEY  ,
	date_created         DATE NOT NULL    ,
	name                 TEXT NOT NULL    ,
	email                TEXT     ,
	phone_number         TEXT     
 );

---------------------------------------------------
---                  discount                   ---
---------------------------------------------------
CREATE TABLE discount ( 
	id                   INT NOT NULL  PRIMARY KEY  ,
	discount_type        TEXT     ,
	amount               INT     
 );

---------------------------------------------------
---                  employee                   ---
---------------------------------------------------
CREATE TABLE employee ( 
	id                   INT NOT NULL  PRIMARY KEY  ,
	employee_number      TEXT NOT NULL    ,
	name                 TEXT NOT NULL    ,
	date_created         DATE NOT NULL    ,
	email                TEXT     ,
	phone_number         TEXT     ,
	title                TEXT     
 );

---------------------------------------------------
---                     tag                     ---
---------------------------------------------------
CREATE TABLE tag ( 
	id                   INT NOT NULL  PRIMARY KEY  ,
	name                 TEXT     
 );

---------------------------------------------------
---                  tax_class                  ---
---------------------------------------------------
CREATE TABLE tax_class ( 
	id                   INT NOT NULL  PRIMARY KEY  ,
	type                 TEXT NOT NULL    ,
	amount               DECIMAL(16) NOT NULL    
 );

---------------------------------------------------
---                  assembly                   ---
---------------------------------------------------
CREATE TABLE assembly ( 
	id                   INT NOT NULL  PRIMARY KEY  ,
	sku                  TEXT     ,
	name                 TEXT NOT NULL    ,
	cost                 DECIMAL(255)     ,
	price                DECIMAL(256)     ,
	discount_id          INT     ,
	tax_class_id         INT     ,
	tag_id               INT     ,
	FOREIGN KEY ( discount_id ) REFERENCES discount( id )  ,
	FOREIGN KEY ( tax_class_id ) REFERENCES tax_class( id )  
 );

---------------------------------------------------
---                assembly_tag                 ---
---------------------------------------------------
CREATE TABLE assembly_tag ( 
	id                   INT     ,
	id_001               INT     ,
	CONSTRAINT pk_assembly_tag UNIQUE ( id, id_001 ),
	FOREIGN KEY ( id ) REFERENCES tag( id )  ,
	FOREIGN KEY ( id_001 ) REFERENCES assembly( id )  
 );

---------------------------------------------------
---                   labour                    ---
---------------------------------------------------
CREATE TABLE labour ( 
	id                   INT NOT NULL  PRIMARY KEY  ,
	sku                  TEXT     ,
	name                 TEXT NOT NULL    ,
	cost                 DECIMAL(255)     ,
	price                DECIMAL(256)     ,
	duration             INT     ,
	discount_id          INT     ,
	tax_class_id         INT NOT NULL    ,
	tag_id               INT     ,
	FOREIGN KEY ( discount_id ) REFERENCES discount( id )  ,
	FOREIGN KEY ( tax_class_id ) REFERENCES tax_class( id )  
 );

---------------------------------------------------
---                 labour_tag                  ---
---------------------------------------------------
CREATE TABLE labour_tag ( 
	id                   INT     ,
	id_001               INT     ,
	CONSTRAINT pk_labour_tag UNIQUE ( id, id_001 ),
	FOREIGN KEY ( id ) REFERENCES tag( id )  ,
	FOREIGN KEY ( id_001 ) REFERENCES labour( id )  
 );

---------------------------------------------------
---                   vendor                    ---
---------------------------------------------------
CREATE TABLE vendor ( 
	id                   INT NOT NULL  PRIMARY KEY  ,
	name                 TEXT NOT NULL    
 );

---------------------------------------------------
---                    item                     ---
---------------------------------------------------
CREATE TABLE item ( 
	id                   INT NOT NULL  PRIMARY KEY  ,
	sku                  TEXT     ,
	name                 TEXT NOT NULL    ,
	cost                 DECIMAL(255)     ,
	price                DECIMAL(256)     ,
	quantity             INT     ,
	discount_id          INT     ,
	tax_class_id         INT NOT NULL    ,
	vendor_id            INT     ,
	tag_id               INT     ,
	FOREIGN KEY ( discount_id ) REFERENCES discount( id )  ,
	FOREIGN KEY ( vendor_id ) REFERENCES vendor( id )  ,
	FOREIGN KEY ( tax_class_id ) REFERENCES tax_class( id )  
 );

---------------------------------------------------
---                     box                     ---
---------------------------------------------------
CREATE TABLE box ( 
	id                   INT NOT NULL  PRIMARY KEY  ,
	sku                  TEXT     ,
	name                 TEXT NOT NULL    ,
	cost                 DECIMAL(255)     ,
	price                DECIMAL(256)     ,
	quantity             INT     ,
	contained_item_id    INT NOT NULL    ,
	contained_quantity   INT     ,
	discount_id          INT     ,
	tax_class_id         INT NOT NULL    ,
	vendor_id            INT     ,
	tag_id               INT     ,
	FOREIGN KEY ( discount_id ) REFERENCES discount( id )  ,
	FOREIGN KEY ( vendor_id ) REFERENCES vendor( id )  ,
	FOREIGN KEY ( tax_class_id ) REFERENCES tax_class( id )  ,
	FOREIGN KEY ( contained_item_id ) REFERENCES item( id )  
 );

---------------------------------------------------
---                   box_tag                   ---
---------------------------------------------------
CREATE TABLE box_tag ( 
	id                   INT     ,
	id_001               INT     ,
	CONSTRAINT pk_box_tag UNIQUE ( id, id_001 ),
	FOREIGN KEY ( id ) REFERENCES tag( id )  ,
	FOREIGN KEY ( id_001 ) REFERENCES box( id )  
 );

---------------------------------------------------
---                  item_tag                   ---
---------------------------------------------------
CREATE TABLE item_tag ( 
	id                   INT     ,
	id_001               INT     ,
	CONSTRAINT pk_item_tag UNIQUE ( id, id_001 ),
	FOREIGN KEY ( id ) REFERENCES tag( id )  ,
	FOREIGN KEY ( id_001 ) REFERENCES item( id )  
 );

---------------------------------------------------
---             junc_assembly_item              ---
---------------------------------------------------
CREATE TABLE junc_assembly_item ( 
	assembly_id          INT NOT NULL    ,
	item_id              INT NOT NULL    ,
	quantity             INT NOT NULL    ,
	CONSTRAINT pk_junc_assembly_item PRIMARY KEY ( assembly_id, item_id ),
	FOREIGN KEY ( assembly_id ) REFERENCES assembly( id )  ,
	FOREIGN KEY ( item_id ) REFERENCES item( id )  
 );

---------------------------------------------------
---               purchase_order                ---
---------------------------------------------------
CREATE TABLE purchase_order ( 
	id                   INT NOT NULL  PRIMARY KEY  ,
	order_id             TEXT NOT NULL    ,
	name                 TEXT NOT NULL    ,
	vendor_id            INT NOT NULL    ,
	order_date           DATE NOT NULL    ,
	arrival_date         DATE     ,
	status               TEXT     ,
	item_id              INT     ,
	box_id               INT     ,
	FOREIGN KEY ( vendor_id ) REFERENCES vendor( id )  ,
	FOREIGN KEY ( item_id ) REFERENCES item( id )  ,
	FOREIGN KEY ( box_id ) REFERENCES box( id )  
 );

---------------------------------------------------
---                    sale                     ---
---------------------------------------------------
CREATE TABLE sale ( 
	id                   INT NOT NULL  PRIMARY KEY  ,
	timestamp            DATETIME NOT NULL    ,
	customer_id          INT NOT NULL    ,
	employee_id          INT NOT NULL    ,
	discount_id          INT     ,
	box_id               INT     ,
	item_id              INT     ,
	assembly_id          INT     ,
	labour_id            INT     ,
	service_order_id     INT     ,
	FOREIGN KEY ( discount_id ) REFERENCES discount( id )  ,
	FOREIGN KEY ( employee_id ) REFERENCES employee( id )  ,
	FOREIGN KEY ( customer_id ) REFERENCES customer( id )  ,
	FOREIGN KEY ( item_id ) REFERENCES item( id )  ,
	FOREIGN KEY ( box_id ) REFERENCES box( id )  ,
	FOREIGN KEY ( labour_id ) REFERENCES labour( id )  ,
	FOREIGN KEY ( assembly_id ) REFERENCES assembly( id )  
 );

---------------------------------------------------
---                   service                   ---
---------------------------------------------------
CREATE TABLE service ( 
	id                   INT NOT NULL  PRIMARY KEY  ,
	name                 TEXT NOT NULL    ,
	description          TEXT     ,
	category_id          INT     ,
	employee_id          INT     ,
	item_id              INT     ,
	assembly_id          INT     ,
	labour_id            INT     ,
	FOREIGN KEY ( category_id ) REFERENCES category( id )  ,
	FOREIGN KEY ( employee_id ) REFERENCES employee( id )  ,
	FOREIGN KEY ( item_id ) REFERENCES item( id )  ,
	FOREIGN KEY ( assembly_id ) REFERENCES assembly( id )  ,
	FOREIGN KEY ( labour_id ) REFERENCES labour( id )  
 );

---------------------------------------------------
---                 appointment                 ---
---------------------------------------------------
CREATE TABLE appointment ( 
	id                   INT NOT NULL  PRIMARY KEY  ,
	appointment_number   INT NOT NULL    ,
	date_taken           DATE NOT NULL    ,
	date_occurring       DATE NOT NULL    ,
	approved             NUMERIC NOT NULL DEFAULT FALSE   ,
	service_id           INT NOT NULL    ,
	customer_id          INT NOT NULL    ,
	FOREIGN KEY ( customer_id ) REFERENCES customer( id )  ,
	FOREIGN KEY ( service_id ) REFERENCES service( id )  
 );

---------------------------------------------------
---                service_order                ---
---------------------------------------------------
CREATE TABLE service_order ( 
	id                   INT NOT NULL  PRIMARY KEY  ,
	date_created         DATE NOT NULL    ,
	date_due             DATE     ,
	employee_id          INT NOT NULL    ,
	customer_id          INT NOT NULL    ,
	notes_receipt        TEXT     ,
	notes_internal       TEXT     ,
	item_id              INT     ,
	labour_id            INT     ,
	assembly_id          INT     ,
	FOREIGN KEY ( item_id ) REFERENCES item( id )  ,
	FOREIGN KEY ( labour_id ) REFERENCES labour( id )  ,
	FOREIGN KEY ( assembly_id ) REFERENCES assembly( id )  ,
	FOREIGN KEY ( employee_id ) REFERENCES employee( id )  ,
	FOREIGN KEY ( customer_id ) REFERENCES customer( id )  
 );

