// @generated automatically by Diesel CLI.

diesel::table! {
    do_dose (do_nummer, do_ra_nummer, do_ra_stockwerk) {
        do_nummer -> Text,
        do_ra_nummer -> Integer,
        do_ra_stockwerk -> Integer,
    }
}

diesel::table! {
    ge_gebaeude (ge_name) {
        ge_name -> Text,
    }
}

diesel::table! {
    ra_raum (ra_ge_name, ra_stockwerk, ra_nummer) {
        ra_ge_name -> Text,
        ra_stockwerk -> Integer,
        ra_nummer -> Text,
    }
}

diesel::table! {
    sc_schrank (sc_nummer, sc_stockwerk, sc_ge_name) {
        sc_nummer -> Integer,
        sc_stockwerk -> Integer,
        sc_ge_name -> Text,
    }
}

diesel::table! {
    sd_switch_zu_dose (sd_sw_ip, sd_do_nummer, sd_do_ra_nummer, sd_do_ra_stockwerk) {
        sd_sw_ip -> Integer,
        sd_do_nummer -> Text,
        sd_do_ra_nummer -> Integer,
        sd_do_ra_stockwerk -> Integer,
        sd_switchport -> Text,
        sd_hat_telefon -> Integer,
        sd_hat_pc -> Integer,
        sd_hat_drucker -> Integer,
        sd_kommentar -> Nullable<Text>,
    }
}

diesel::table! {
    sw_switch (sw_ip) {
        sw_ip -> Text,
        sw_sc_nummer -> Integer,
        sw_sc_stockwerk -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    do_dose,
    ge_gebaeude,
    ra_raum,
    sc_schrank,
    sd_switch_zu_dose,
    sw_switch,
);
