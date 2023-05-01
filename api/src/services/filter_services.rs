#[macro_export]
macro_rules! filter_date_time {
    ($column:expr, $value:expr, $query:expr) => {
        if let Some(start) = $value.start {
            $query = if $value.include_start {
                $query.filter($column.ge(start))
            } else {
                $query.filter($column.gt(start))
            };
        }
        if let Some(end) = $value.end {
            $query = if $value.include_end {
                $query.filter($column.le(end))
            } else {
                $query.filter($column.lt(end))
            };
        }
    };
}
