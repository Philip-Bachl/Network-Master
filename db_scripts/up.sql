PRAGMA foreign_keys = 0;

CREATE TABLE ge_gebaeude (
    ge_name TEXT NOT NULL,
    PRIMARY KEY (ge_name)
);

CREATE TABLE ra_raum (
    ra_nummer TEXT NOT NULL,
    ra_stockwerk INTEGER NOT NULL,
    ra_ge_name TEXT NOT NULL,
    PRIMARY KEY (ra_ge_name, ra_stockwerk, ra_nummer),
    FOREIGN KEY (ra_ge_name) REFERENCES ge_gebaeude(ge_name)
);

CREATE TABLE sc_schrank (
    sc_nummer TEXT NOT NULL,
    sc_stockwerk INTEGER NOT NULL,
    sc_ge_name TEXT NOT NULL,
    PRIMARY KEY (sc_nummer, sc_stockwerk, sc_ge_name)
    FOREIGN KEY (sc_ge_name) REFERENCES ge_gebaeude(ge_name)
);

CREATE TABLE do_dose (
    do_nummer TEXT NOT NULL,
    do_ra_nummer TEXT NOT NULL,
    do_ra_stockwerk INTEGER NOT NULL,
    do_ra_ge_name TEXT NOT NULL,
    PRIMARY KEY (do_nummer, do_ra_ge_name, do_ra_nummer, do_ra_stockwerk)
    FOREIGN KEY (do_ra_nummer, do_ra_stockwerk, do_ra_ge_name) REFERENCES ra_raum(ra_nummer, ra_stockwerk, ra_ge_name)
);

CREATE TABLE sw_switch (
    sw_ip TEXT NOT NULL,
    sw_sc_nummer TEXT NOT NULL,
    sw_sc_stockwerk INTEGER NOT NULL,
    sw_sc_ge_name TEXT NOT NULL,
    PRIMARY KEY (sw_ip)
    FOREIGN KEY (sw_sc_nummer, sw_sc_stockwerk, sw_sc_ge_name) REFERENCES sc_schrank(sc_nummer, sc_stockwerk, sc_ge_name)
);

CREATE TABLE sd_switch_zu_dose (
    sd_sw_ip TEXT NOT NULL,
    sd_do_nummer TEXT NOT NULL,
    sd_do_ra_nummer TEXT NOT NULL,
    sd_do_ra_stockwerk INTEGER NOT NULL,
    sd_do_ra_ge_name TEXT NOT NULL,
    sd_switchport TEXT NOT NULL,
    sd_hat_telefon BOOLEAN NOT NULL,
    sd_hat_pc BOOLEAN NOT NULL,
    sd_hat_drucker BOOLEAN NOT NULL,
    sd_kommentar TEXT,
    PRIMARY KEY (sd_sw_ip, sd_do_nummer, sd_do_ra_nummer, sd_do_ra_stockwerk, sd_do_ra_ge_name)
    FOREIGN KEY (sd_sw_ip) REFERENCES sw_switch(sw_ip),
    FOREIGN KEY (sd_do_nummer, sd_do_ra_nummer, sd_do_ra_stockwerk, sd_do_ra_ge_name) REFERENCES do_dose(do_nummer, do_ra_nummer, do_ra_stockwerk, do_ra_ge_name)
);

PRAGMA foreign_keys = 1;