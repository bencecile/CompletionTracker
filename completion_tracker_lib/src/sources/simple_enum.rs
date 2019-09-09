pub trait SimpleEnum
where Self: Sized + Copy + Clone + 'static {
    fn all() -> &'static [Self];
    fn as_str(&self) -> &'static str;
    
    fn from_str(string: &str) -> Option<Self> {
        for simple_enum in Self::all() {
            if simple_enum.as_str() == string {
                return Some(*simple_enum);
            }
        }
        None
    }
}
/// Implements ToSql and FromSql for types implementing SimpleEnum
#[macro_export]
macro_rules! impl_sql_simple_enum {
    ($type: ty) => {
        impl ::rusqlite::types::ToSql for $type {
            fn to_sql(&self) -> ::rusqlite::Result<::rusqlite::types::ToSqlOutput> {
                self.as_str().to_sql()
            }
        }
        impl ::rusqlite::types::FromSql for $type {
            fn column_result(value: ::rusqlite::types::ValueRef)
            -> ::rusqlite::types::FromSqlResult<Self> {
                match String::column_result(value) {
                    Ok(string) => match Self::from_str(&string) {
                        Some(simple_enum) => Ok(simple_enum),
                        None => Err(::rusqlite::types::FromSqlError::InvalidType),
                    },
                    Err(e) => Err(e),
                }
            }
        }
    }
}
