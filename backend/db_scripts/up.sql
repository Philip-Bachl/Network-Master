PRAGMA foreign_keys = 0;

CREATE TABLE ge_gebaeude (
    ge_name TEXT NOT NULL,
    ge_kommentar TEXT,
    PRIMARY KEY (ge_name)
);

CREATE TABLE ra_raum (
    ra_id INTEGER PRIMARY KEY,
    ra_ge_name TEXT NOT NULL,
    ra_nummer TEXT NOT NULL,
    ra_stockwerk INTEGER NOT NULL,
    ra_kommentar TEXT,

    UNIQUE (ra_ge_name, ra_nummer, ra_stockwerk),
    FOREIGN KEY (ra_ge_name) REFERENCES ge_gebaeude(ge_name)
);

CREATE TABLE sc_schrank (
    sc_id INTEGER PRIMARY KEY,
    sc_ge_name TEXT NOT NULL,
    sc_nummer TEXT NOT NULL,
    sc_stockwerk INTEGER NOT NULL,
    sc_kommentar TEXT,

    UNIQUE (sc_ge_name, sc_nummer, sc_stockwerk),
    FOREIGN KEY (sc_ge_name) REFERENCES ge_gebaeude(ge_name)
);

CREATE TABLE dk_device_kind (
    dk_id INTEGER PRIMARY KEY,
    dk_name TEXT NOT NULL,
    dk_kommentar TEXT,

    UNIQUE (dk_name)
);

CREATE TABLE do_dose (
    do_id INTEGER PRIMARY KEY,
    do_ra_id INTEGER NOT NULL,
    do_nummer TEXT NOT NULL,
    do_sp_id INTEGER,
    do_dk_id INTEGER,
    do_kommentar TEXT,

    UNIQUE (do_ra_id, do_nummer),
    FOREIGN KEY (do_ra_id) REFERENCES ra_raum(ra_id),
    FOREIGN KEY (do_sp_id) REFERENCES sp_switchport(sp_id),
    FOREIGN KEY (do_dk_id) REFERENCES dk_device_kind(dk_id)
);

CREATE TABLE sw_switch (
    sw_id INTEGER PRIMARY KEY,
    sw_name TEXT NOT NULL,
    sw_sc_id INTEGER NOT NULL,
    sw_ip TEXT NOT NULL,
    sw_kommentar TEXT,
    
    UNIQUE (sw_name),
    FOREIGN KEY (sw_sc_id) REFERENCES sc_schrank(sc_id)
);

CREATE TABLE sp_switchport (
    sp_id INTEGER PRIMARY KEY,
    sp_sw_id INTEGER NOT NULL,
    sp_port TEXT NOT NULL,
    sp_vlan INTEGER NOT NULL,
    sp_dot1x BOOLEAN NOT NULL,
    sp_kommentar TEXT,

    UNIQUE (sp_sw_id, sp_port),
    FOREIGN KEY (sp_sw_id) REFERENCES sw_switch(sw_id)
);

PRAGMA foreign_keys = 1;