table! {
    posts (datetime) {
        datetime -> Timestamptz,
        temperature -> Nullable<Float4>,
        relative_humidity -> Nullable<Float4>,
        atmospheric_pressure -> Nullable<Float4>,
    }
}
