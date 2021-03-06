table! {
    audit_logs (id) {
        id -> Varchar,
        kind -> Int2,
        user_id -> Varchar,
        room_id -> Varchar,
        table_name -> Varchar,
        changes -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    channels (id) {
        id -> Varchar,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

table! {
    dm_channels (id) {
        id -> Varchar,
        channel_id -> Varchar,
    }
}

table! {
    dm_channel_users (id) {
        id -> Varchar,
        user_id -> Varchar,
        dm_channel_id -> Varchar,
    }
}

table! {
    emotes (id) {
        id -> Varchar,
        name -> Varchar,
        file_id -> Varchar,
        room_id -> Varchar,
        is_global -> Bool,
        is_deleted -> Bool,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

table! {
    files (id) {
        id -> Varchar,
        hash -> Varchar,
        ext -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    message_mentions (id) {
        id -> Varchar,
        user_id -> Varchar,
        message_id -> Varchar,
    }
}

table! {
    messages (id) {
        id -> Varchar,
        channel_id -> Varchar,
        user_id -> Varchar,
        content -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    restrains (id) {
        id -> Varchar,
        user_id -> Varchar,
        ip -> Nullable<Varchar>,
        fingerprint -> Nullable<Varchar>,
        channel_id -> Nullable<Varchar>,
        is_global -> Bool,
        is_ban -> Bool,
        ending_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

table! {
    roles (id) {
        id -> Varchar,
        room_id -> Varchar,
        name -> Varchar,
        color -> Nullable<Varchar>,
        is_default -> Bool,
        position -> Int4,
        title_update -> Int4,
        path_update -> Int4,
        public_update -> Int4,
        room_delete -> Int4,
        room_view -> Int4,
        audit_log_read -> Int4,
        embed_links -> Int4,
        ping_everyone -> Int4,
        password_create -> Int4,
        password_update -> Int4,
        password_delete -> Int4,
        password_bypass -> Int4,
        emote_create -> Int4,
        emote_update -> Int4,
        emote_delete -> Int4,
        emote_view -> Int4,
        role_create -> Int4,
        role_delete -> Int4,
        role_update -> Int4,
        role_view -> Int4,
        video_create -> Int4,
        video_delete -> Int4,
        video_watch -> Int4,
        video_move -> Int4,
        video_iframe -> Int4,
        video_raw -> Int4,
        player_pause -> Int4,
        player_resume -> Int4,
        player_rewind -> Int4,
        subtitles_file -> Int4,
        subtitles_embed -> Int4,
        message_create -> Int4,
        message_read -> Int4,
        message_delete -> Int4,
        message_history_read -> Int4,
        message_timeout -> Int4,
        user_kick -> Int4,
        user_ban -> Int4,
        user_unban -> Int4,
        user_timeout -> Int4,
        created_at -> Timestamp,
    }
}

table! {
    room_channels (id) {
        id -> Varchar,
        channel_id -> Varchar,
        room_id -> Varchar,
    }
}

table! {
    rooms (id) {
        id -> Varchar,
        title -> Varchar,
        path -> Varchar,
        is_public -> Bool,
        is_deleted -> Bool,
        password -> Nullable<Varchar>,
        created_at -> Timestamp,
        last_login -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    subtitles (id) {
        id -> Varchar,
        file_id -> Varchar,
        url -> Nullable<Varchar>,
    }
}

table! {
    user_roles (id) {
        id -> Varchar,
        role_id -> Varchar,
        user_id -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Varchar,
        discord_id -> Nullable<Varchar>,
        username -> Varchar,
        nickname -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        color -> Nullable<Varchar>,
        file_id -> Nullable<Varchar>,
        is_admin -> Bool,
        last_login -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

table! {
    videos (id) {
        id -> Varchar,
        room_id -> Varchar,
        subtitles_id -> Nullable<Varchar>,
        file_id -> Nullable<Varchar>,
        url -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        duration -> Nullable<Int4>,
        is_raw -> Bool,
        is_iframe -> Bool,
        is_live -> Bool,
        created_at -> Timestamp,
    }
}

joinable!(audit_logs -> rooms (room_id));
joinable!(audit_logs -> users (user_id));
joinable!(dm_channel_users -> dm_channels (dm_channel_id));
joinable!(dm_channel_users -> users (user_id));
joinable!(dm_channels -> channels (channel_id));
joinable!(emotes -> files (file_id));
joinable!(emotes -> rooms (room_id));
joinable!(message_mentions -> messages (message_id));
joinable!(message_mentions -> users (user_id));
joinable!(messages -> channels (channel_id));
joinable!(messages -> users (user_id));
joinable!(restrains -> channels (channel_id));
joinable!(restrains -> users (user_id));
joinable!(roles -> rooms (room_id));
joinable!(room_channels -> channels (channel_id));
joinable!(room_channels -> rooms (room_id));
joinable!(subtitles -> files (file_id));
joinable!(user_roles -> roles (role_id));
joinable!(user_roles -> users (user_id));
joinable!(users -> files (file_id));
joinable!(videos -> rooms (room_id));

allow_tables_to_appear_in_same_query!(
    audit_logs,
    channels,
    dm_channels,
    dm_channel_users,
    emotes,
    files,
    message_mentions,
    messages,
    restrains,
    roles,
    room_channels,
    rooms,
    subtitles,
    user_roles,
    users,
    videos,
);
