-- Your SQL goes here
CREATE TABLE ge_gebaeude (
    ge_name TEXT,
    PRIMARY KEY (ge_name)
);

CREATE TABLE ra_raum (
    ra_ge_name TEXT REFERENCES ge_gebaeude.ge_name,
    ra_stockwerk INTEGER,
    ra_nummer TEXT,
    PRIMARY KEY (ra_ge_name, ra_stockwerk, ra_nummer)
);

CREATE TABLE sc_schrank (
    sc_nummer INTEGER,
    sc_stockwerk INTEGER,
    sc_ge_name TEXT,
    PRIMARY KEY (sc_nummer, sc_stockwerk, sc_ge_name)
);

CREATE TABLE do_dose (
    do_nummer TEXT,
    do_ra_nummer INTEGER REFERENCES ra_raum.ra_nummer,
    do_ra_stockwerk INTEGER REFERENCES ra_raum.ra_stockwerk,
    PRIMARY KEY (do_nummer, do_ra_nummer, do_ra_stockwerk)
);

CREATE TABLE sw_switch (
    sw_ip TEXT,
    sw_sc_nummer INTEGER REFERENCES sc_schrank.sc_nummer,
    sw_sc_stockwerk INTEGER REFERENCES sc_schrank.sc_stockwerk,
    PRIMARY KEY (sw_ip)
);

-- TODO: SD_Switch_zu_Dose