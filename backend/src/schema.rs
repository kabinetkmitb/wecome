table! {
    identitas_pengaju (id) {
        id -> Text,
        kontak -> Text,
        pengaju_id -> Text,
    }
}

table! {
    kompetisi (id) {
        id -> Text,
        nama_lembaga_pendaftar -> Text,
        no_telp -> Text,
        nama_kompetisi -> Text,
        kategori_kompetisi -> Text,
        deskripsi_kompetisi -> Text,
        tags_kompetisi -> Text,
        tanggal_pelaksanaan -> Timestamp,
        batas_awal_registrasi -> Timestamp,
        batas_akhir_registrasi -> Timestamp,
        link_registrasi -> Text,
        link_website -> Text,
        link_linkedin -> Text,
        akun_instagram -> Text,
        id_line -> Text,
        akun_twitter -> Text,
        link_poster -> Text,
        status_kompetisi -> Text,
        user_id -> Text,
        created_at -> Timestamp,
    }
}

table! {
    kriteria_anggota (id) {
        id -> Text,
        kompetisi_id -> Text,
        role -> Text,
        deskripsi_kriteria -> Text,
    }
}

table! {
    tim_buddies (id) {
        id -> Text,
        kompetisi_id -> Text,
    }
}

table! {
    user (id) {
        id -> Text,
        email -> Text,
        name -> Text,
        nim -> Text,
        password -> Text,
        is_admin -> Bool,
        is_active -> Bool,
    }
}

table! {
    verification (id) {
        id -> Text,
        user_id -> Text,
    }
}

joinable!(identitas_pengaju -> user (pengaju_id));
joinable!(kompetisi -> user (user_id));
joinable!(kriteria_anggota -> kompetisi (kompetisi_id));
joinable!(tim_buddies -> kompetisi (kompetisi_id));
joinable!(verification -> user (user_id));

allow_tables_to_appear_in_same_query!(
    identitas_pengaju,
    kompetisi,
    kriteria_anggota,
    tim_buddies,
    user,
    verification,
);
