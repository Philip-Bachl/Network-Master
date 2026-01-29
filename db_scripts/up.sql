PRAGMA foreign_keys = 0;

CREATE TABLE ge_gebaeude (
    ge_name TEXT NOT NULL,
    PRIMARY KEY (ge_name)
);

CREATE TABLE ra_raum (
    ra_nummer TEXT NOT NULL,
    ra_stockwerk INTEGER NOT NULL,
    ra_ge_name TEXT REFERENCES ge_gebaeude(ge_name) ON DELETE SET NULL ON UPDATE SET NULL,
    PRIMARY KEY (ra_ge_name, ra_stockwerk, ra_nummer)
);

CREATE TABLE sc_schrank (
    sc_nummer INTEGER NOT NULL,
    sc_stockwerk INTEGER NOT NULL,
    sc_ge_name TEXT REFERENCES ge_gebaeude(ge_name) ON DELETE SET NULL ON UPDATE SET NULL,
    PRIMARY KEY (sc_nummer, sc_stockwerk, sc_ge_name)
);

CREATE TABLE do_dose (
    do_nummer TEXT NOT NULL,
    do_ra_ge_name TEXT REFERENCES ra_raum(ra_ge_name) ON DELETE SET NULL ON UPDATE SET NULL,
    do_ra_nummer INTEGER REFERENCES ra_raum(ra_nummer) ON DELETE SET NULL ON UPDATE SET NULL,
    do_ra_stockwerk INTEGER REFERENCES ra_raum(ra_stockwerk) ON DELETE SET NULL ON UPDATE SET NULL,
    PRIMARY KEY (do_nummer, do_ra_ge_name, do_ra_nummer, do_ra_stockwerk)
);

CREATE TABLE sw_switch (
    sw_ip TEXT NOT NULL,
    sw_sc_nummer INTEGER NOT NULL,
    sw_sc_stockwerk INTEGER NOT NULL,
    sw_sc_ge_name TEXT REFERENCES sc_schrank(sc_ge_name) ON DELETE SET NULL ON UPDATE SET NULL,
    PRIMARY KEY (sw_ip)
);

CREATE TABLE sd_switch_zu_dose (
    sd_sw_ip TEXT REFERENCES sw_switch(sw_ip) ON DELETE SET NULL ON UPDATE SET NULL,
    sd_do_nummer TEXT REFERENCES do_dose(do_nummer) ON DELETE SET NULL ON UPDATE SET NULL,
    sd_do_ra_nummer INTEGER REFERENCES do_dose(do_ra_nummer) ON DELETE SET NULL ON UPDATE SET NULL,
    sd_do_ra_stockwerk INTEGER REFERENCES do_dose(do_ra_stockwerk) ON DELETE SET NULL ON UPDATE SET NULL,
    sd_do_ra_ge_name TEXT REFERENCES do_dose(do_ra_ge_name) ON DELETE SET NULL ON UPDATE SET NULL,
    sd_switchport TEXT NOT NULL,
    sd_hat_telefon BOOLEAN NOT NULL,
    sd_hat_pc BOOLEAN NOT NULL,
    sd_hat_drucker BOOLEAN NOT NULL,
    sd_kommentar TEXT,
    PRIMARY KEY (sd_sw_ip, sd_do_nummer, sd_do_ra_nummer, sd_do_ra_stockwerk, sd_do_ra_ge_name)
);

PRAGMA foreign_keys = 1;