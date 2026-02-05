PRAGMA foreign_keys = 0;

CREATE TABLE ge_gebaeude (
    ge_name TEXT NOT NULL,
    PRIMARY KEY (ge_name)
);

CREATE TABLE ra_raum (
    ra_id INTEGER PRIMARY KEY,
    ra_ge_name TEXT NOT NULL,
    ra_nummer TEXT NOT NULL,
    ra_stockwerk INTEGER NOT NULL,

    UNIQUE (ra_ge_name, ra_nummer, ra_stockwerk),
    FOREIGN KEY (ra_ge_name) REFERENCES ge_gebaeude(ge_name)
);

CREATE TABLE sc_schrank (
    sc_id INTEGER PRIMARY KEY,
    sc_ge_name TEXT NOT NULL,
    sc_nummer TEXT NOT NULL,
    sc_stockwerk INTEGER NOT NULL,

    UNIQUE (sc_nummer, sc_ge_name, sc_stockwerk),
    FOREIGN KEY (sc_ge_name) REFERENCES ge_gebaeude(ge_name)
);

CREATE TABLE do_dose (
    do_id INTEGER PRIMARY KEY,
    do_ra_id INTEGER NOT NULL,
    do_nummer TEXT NOT NULL,
    do_hat_telefon BOOLEAN NOT NULL,
    do_hat_pc BOOLEAN NOT NULL,
    do_hat_drucker BOOLEAN NOT NULL,

    UNIQUE (do_ra_id, do_nummer),
    FOREIGN KEY (do_ra_id) REFERENCES ra_raum(ra_id)
);

CREATE TABLE sw_switch (
    sw_name TEXT NOT NULL,
    sw_sc_id INTEGER NOT NULL,
    sw_ip TEXT NOT NULL,
    
    PRIMARY KEY (sw_name),
    FOREIGN KEY (sw_sc_id) REFERENCES sc_schrank(sc_id)
);

CREATE TABLE szd_switch_zu_dose (
    szd_sw_name TEXT,
    szd_do_id INTEGER,
    szd_port TEXT,
    szd_vlan INTEGER,
    szd_Kommentar TEXT,

    PRIMARY KEY (szd_sw_name, szd_do_id),
    FOREIGN KEY (szd_sw_name) REFERENCES sw_switch(sw_name),
    FOREIGN KEY (szd_do_id) REFERENCES do_dose(do_id)
);

PRAGMA foreign_keys = 1;