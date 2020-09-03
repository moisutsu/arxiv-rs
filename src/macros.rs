#[macro_export]
macro_rules! query {
    ( $($i:ident = $e:expr),* ) => {
        {
            let temp_query = $crate::ArxivQueryBuilder::new();
            $(
                $crate::query_inner!($i, $e, temp_query);
            )*
            temp_query.build()
        }
    }
}

#[macro_export]
macro_rules! query_inner {
    (search_query, $e:expr, $temp_query: ident) => {
        let $temp_query = $temp_query.search_query($e);
    };
    (start, $e:expr, $temp_query: ident) => {
        let $temp_query = $temp_query.start($e);
    };
    (max_results, $e:expr, $temp_query: ident) => {
        let $temp_query = $temp_query.max_results($e);
    };
    (sort_by, $e:expr, $temp_query: ident) => {
        let $temp_query = $temp_query.sort_by($e);
    };
    (sort_order, $e:expr, $temp_query: ident) => {
        let $temp_query = $temp_query.sort_order($e);
    };
}
