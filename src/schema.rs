// @generated automatically by Diesel CLI.

diesel::table! {
    do_dose (do_nummer, do_ra_nummer, do_ra_stockwerk) {
        do_nummer -> Nullable<Text>,
        do_ra_nummer -> Nullable<Integer>,
        do_ra_stockwerk -> Nullable<Integer>,
    }
}

diesel::table! {
    ge_gebaeude (ge_name) {
        ge_name -> Nullable<Text>,
    }
}

diesel::table! {
    ra_raum (ra_ge_name, ra_stockwerk, ra_nummer) {
        ra_ge_name -> Nullable<Text>,
        ra_stockwerk -> Nullable<Integer>,
        ra_nummer -> Nullable<Text>,
    }
}

diesel::table! {
    sc_schrank (sc_nummer, sc_stockwerk, sc_ge_name) {
        sc_nummer -> Nullable<Integer>,
        sc_stockwerk -> Nullable<Integer>,
        sc_ge_name -> Nullable<Text>,
    }
}

diesel::table! {
    sd_switch_zu_dose (sd_sw_ip, sd_do_nummer, sd_do_ra_nummer, sd_do_ra_stockwerk) {
        sd_sw_ip -> Nullable<Integer>,
        sd_do_nummer -> Nullable<Text>,
        sd_do_ra_nummer -> Nullable<Integer>,
        sd_do_ra_stockwerk -> Nullable<Integer>,
        sd_switchport -> Nullable<Text>,
        sd_hat_telefon -> Nullable<Integer>,
        sd_hat_pc -> Nullable<Integer>,
        sd_hat_drucker -> Nullable<Integer>,
        sd_kommentar -> Nullable<Text>,
    }
}

diesel::table! {
    sw_switch (sw_ip) {
        sw_ip -> Nullable<Text>,
        sw_sc_nummer -> Nullable<Integer>,
        sw_sc_stockwerk -> Nullable<Integer>,
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
