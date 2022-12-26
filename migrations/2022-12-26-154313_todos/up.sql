CREATE TABLE todos(
    id serial NOT NULL, 
    title character varying(255) NOT NULL,
    description text NOT NULL,
    start_time INT NOT NULL,
    end_time INT NOT NULL,
    CONSTRAINT todos_pkey PRIMARY KEY (id)
);