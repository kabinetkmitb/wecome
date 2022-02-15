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
        nama_pendaftar -> Text,
        nim_pendaftar -> Text,
        email_pendaftar -> Text,
        nama_lembaga_pendaftar -> Text,
        nama_kompetisi -> Text,
        kategori_kompetisi -> Text,
        deskripsi_kompetisi -> Text,
        tags_kompetisi -> Text,
        tanggal_pelaksanaan -> Timestamp,
        batas_registrasi -> Timestamp,
        link_registrasi -> Text,
        link_webiste -> Text,
        link_linkedin -> Text,
        akun_instagram -> Text,
        id_line -> Text,
        akun_twitter -> Text,
        link_poster -> Text,
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
    }
}

table! {
    verification (id) {
        id -> Text,
        is_verified -> Bool,
        code -> Text,
        user_id -> Text,
    }
}

joinable!(identitas_pengaju -> user (pengaju_id));
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
